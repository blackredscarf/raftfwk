#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde;

pub mod msg;
pub mod rpc;
pub mod kit;
pub mod proto;
pub mod meta;
pub mod service;
pub mod logger;
pub mod storage;
pub mod conf;
pub mod mem_storage;
pub mod level_storage;

use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{mpsc, Arc, RwLock, Mutex};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::thread;
use std::panic;
use core::borrow::BorrowMut;

use slog::{Logger, Drain};
use raft::{RawNode, Config, Error};
use raft::storage::MemStorage;
use raft::prelude::*;
use protobuf::Message as GBMessage;
use grpcio::{ChannelBuilder, EnvBuilder};

use crate::msg::*;
use crate::rpc::MpcServer;
use crate::rpc::create_rpc_client;
use crate::proto::mpc_grpc::MpcClient;
use crate::proto::mpc::*;
use crate::proto::mmsg::Message;
use crate::meta::ServerMeta;
use crate::service::*;

use crate::storage::RaftStorage;
use crate::msg::{Msg, CommandType, Command};

pub struct RaftConfig {
    id: u64,
    port: u16,
    election_tick: usize,
    heartbeat_tick: usize,
    cluster: Option<String>,
    pre_vote: bool,
    snap_count: u64
}

impl RaftConfig {
    pub fn simple(id: u64, port: u16) -> Self {
        RaftConfig {
            id,
            port,
            election_tick: 10,
            heartbeat_tick: 3,
            cluster: None,
            pre_vote: false,
            snap_count: 10
        }
    }

    pub fn join(id: u64, port: u16, cluster: Option<String>) -> Self {
        RaftConfig {
            id,
            port,
            election_tick: 10,
            heartbeat_tick: 3,
            cluster: cluster,
            pre_vote: false,
            snap_count: 10
        }
    }

}

pub struct RaftServer<T: RaftStorage> {
    id: u64,
    port: u16,
    cluster: String,
    meta: Arc<RwLock<ServerMeta>>,
    logger: Logger,
    r: RawNode<T>,
    service: Arc<Mutex<dyn RaftService+Send>>,
    context: RaftContext,
    peers: HashMap<u64, String>,
    mpc_server: MpcServer,
    clients: HashMap<u64, MpcClient>,

    snapshot_index: u64,
    applied_index: u64,
    snap_count: u64
}

impl <T: RaftStorage> RaftServer<T> {
    pub fn new(logger: Logger, params: RaftConfig, storage: T, service: Arc<Mutex<dyn RaftService+Send>>) -> Self {

        let cluster = match params.cluster {
            Some(v) => {
                if !kit::check_addr(&v) {
                    panic!("Cluster format error. It should be like 0.0.0.0:8060");
                }
                v
            },
            None => {
                storage.initialize_with_conf_state(ConfState::from((vec![params.id], vec![])));
                String::from("")
            }
        };

        let last_applied = storage.hard_state().commit;
        let cfg = Config {
            id: params.id,
            election_tick: params.election_tick,
            heartbeat_tick: params.heartbeat_tick,
            pre_vote: params.pre_vote,
            max_size_per_msg: 1024 * 1024 * 1024,
            max_inflight_msgs: 256,
            applied: last_applied,
            ..Default::default()
        };

        let r = RawNode::new(&cfg, storage, &logger).unwrap();

        let meta = Arc::new(RwLock::new(ServerMeta::default()));
        let mpc_server = MpcServer::new(logger.clone(), params.id, params.port, meta.clone(), service.clone(), 1);

        let mut peers = HashMap::new();
        peers.insert(params.id, format!("127.0.0.1:{}", params.port));

        RaftServer {
            id: params.id,
            port: params.port,
            cluster,
            logger,
            r,
            service,
            mpc_server,
            peers,
            meta,
            snap_count: params.snap_count,
            snapshot_index: last_applied,
            applied_index: last_applied,
            context: RaftContext::new(params.id),
            clients: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        self.mpc_server.run();
        thread::sleep(Duration::from_secs(2));

        let mut t = Instant::now();
        let mut t2 = Instant::now();
        let mut msg_id = 0;

        if !self.cluster.is_empty() {
            self.join();
        }

        loop {
            loop {
                let res = self.service.lock().unwrap().recv();
                match res {
                    Ok(Msg::Propose(normal)) => {
                        match self.r.propose(vec![], normal.data) {
                            Ok(v) => (),
                            Err(e) => warn!(self.logger, "Fail to propose: {:?}", e)
                        }

                    },
                    Ok(Msg::Command(cmd)) => {
                        match cmd.command {
                            CommandType::Join => {
                                if let Some(host) = cmd.host {
                                    let addr = format!("{}:{}", host, cmd.port);
                                    self.add_follower(cmd.id, &addr);
                                }
                            }
                        }
                    }
                    Ok(Msg::ConfChange(cc)) => {
                        debug!(self.logger, "Conf change {:?}", cc);
                        self.r.propose_conf_change(vec![], cc);
                    }
                    Ok(Msg::Raft(m)) => {
                        match self.r.step(m) {
                            Ok(v) => (),
                            // Probably cause when removed a node
                            Err(Error::StepPeerNotFound) => (),
                            Err(e) => warn!(self.logger, "Step error: {}", e)
                        }
                    },
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }

            if t.elapsed() >= Duration::from_millis(500) {
                // Tick the raft.
                self.r.tick();
                t = Instant::now();

                // Update metadata
                {
                    let mut meta_g = self.meta.write().unwrap();
                    meta_g.borrow_mut().leader_id = self.r.raft.leader_id;
                    self.context.leader_id = self.r.raft.leader_id;
                }
                self.context.peers = self.peers.clone();
                self.service.lock().unwrap().update_context(self.context.clone());
            }

            self.on_ready();
        }
    }

    fn on_ready(&mut self) {
        if !self.r.has_ready() {
            return;
        }

        let mut ready = self.r.ready();
        let is_leader = self.r.raft.leader_id == self.r.raft.id;

        if *ready.snapshot() != Snapshot::default() {
            match self.r.mut_store().apply_snapshot(ready.snapshot().clone()) {
                Ok(v) => (),
                Err(e) => error!(self.logger, "Fail to snapshot: {:?}", e)
            }
        }

        if let Err(e) = self.r.mut_store().append(ready.entries()) {
            error!(self.logger, "Persist raft log fail: {:?}", e);
            return;
        }

        if let Some(hs) = ready.hs() {
            self.r.mut_store().set_hardstate(hs.clone());
        }

        for msg in ready.messages.drain(..) {
            let to = msg.to;
            self.send(msg);
        }

        if let Some(committed_entries) = ready.committed_entries.take() {
            for entry in committed_entries {
                if entry.data.is_empty() {
                    continue;
                }
                if let EntryType::EntryConfChange = entry.get_entry_type() {
                    let mut cc = ConfChange::default();
                    cc.merge_from_bytes(&entry.data).unwrap();
                    let node_id = cc.node_id;
                    let node_addr = String::from_utf8(cc.get_context().to_vec());
                    match cc.get_change_type() {
                        ConfChangeType::AddNode => {
                            if let None = self.r.raft.prs().get(node_id) {
                                match self.r.raft.add_node(node_id) {
                                    Ok(v) => (),
                                    Err(e) => warn!(self.logger, "{}", e)
                                };
                            }
                            self.peers.insert(node_id, node_addr.unwrap());
                        },
                        ConfChangeType::RemoveNode => {
                            match self.r.raft.remove_node(node_id) {
                                Ok(v) => (),
                                Err(e) => warn!(self.logger, "{}", e)
                            };
                            self.peers.remove(&node_id);
                        },
                        ConfChangeType::AddLearnerNode => self.r.raft.add_learner(node_id).unwrap(),
                    }
                    let cs = self.r.raft.prs().configuration().to_conf_state();
                    self.r.mut_store().set_conf_state(cs);
                }
                if entry.get_entry_type() == EntryType::EntryNormal {
                    debug!(self.logger, "Commit");
                    self.service.lock().unwrap().on_committed(&entry.data);
                }
                self.applied_index = entry.index;
            }
        }
        self.maybe_snapshot();
        self.r.advance(ready);
    }

    fn maybe_snapshot(&mut self) {
        if self.applied_index - self.snapshot_index <= self.snap_count {
            return;
        }
        if let Ok(snap) = self.r.mut_store().snapshot(self.applied_index) {
            self.r.mut_store().apply_snapshot(snap);
            self.snapshot_index = self.applied_index;
            info!(self.logger, "Apply snapshot");
        } else {
            warn!(self.logger, "Cannot save the snapshot");
        }
    }

    fn join(&mut self) {
        let cmd = Command::new_join(self.id, self.port);

        let mut msg = Message::default();
        msg.set_context(bincode::serialize(&cmd).unwrap());
        let client = create_rpc_client(&self.cluster);
        match client.send(&msg) {
            Ok(res) => {
                match res.code {
                    MpcReplyCode::Ok => {
                        match bincode::deserialize::<u64>(res.get_context()) {
                            Ok(leader_id) => self.peers.insert(leader_id, self.cluster.clone()),
                            Err(e) => panic!("Fail to join in cluster {}: {}", self.cluster, e),
                        };
                    },
                    MpcReplyCode::ERR => {
                        panic!("Fail to join in cluster {}: {}", self.cluster, res.msg);
                    }
                }
            },
            Err(e) => {
                panic!("Fail to join in cluster {}", self.cluster);
            }
        }
    }

    fn add_follower(&mut self, id: u64, addr: &String) {
        let mut cc = ConfChange::default();
        cc.node_id = id;
        cc.set_change_type(ConfChangeType::AddNode);
        cc.set_context(addr.clone().into_bytes());
        self.service.lock().unwrap().send(Msg::ConfChange(cc));
    }

    fn send(&mut self, msg: Message) {
        if self.clients.get(&msg.to).is_none() {
            let addr = match self.peers.get(&msg.to) {
                Some(n) => n,
                None => {
                    debug!(self.logger, "Cannot find address of id {}", msg.to);
                    return;
                }
            };

            let client = create_rpc_client(addr);
            self.clients.insert(msg.to, client);
            debug!(self.logger, "Create a rpc client to {}", addr);
        }

        if let Some(client) = self.clients.get(&msg.to) {
            debug!(self.logger, "Send {:?}", msg);
            match client.send(&msg) {
                Ok(reply) => {
                    debug!(self.logger, "Reply {:?}", reply);
                },
                Err(e) => {
                    debug!(self.logger, "Rpc error: {:?}", e);
                }
            }
        } else {
            debug!(self.logger, "Cannot find client of id {}", msg.to);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
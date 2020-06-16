use std::thread;
use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use grpcio::{RpcContext, UnarySink, Server, Environment, ResourceQuota, ChannelBuilder, ServerBuilder, EnvBuilder, Service};
use futures_01::future::Future;
use slog::Logger;

use crate::msg::*;
use crate::proto::mmsg::Message;
use crate::proto::mpc_grpc::Mpc;
use crate::proto::mpc_grpc::MpcClient;
use crate::proto::mpc_grpc::create_mpc;
use crate::proto::mpc::*;
use crate::meta::ServerMeta;
use crate::service::*;

#[derive(Clone)]
pub struct MpcService {
    id: u64,
    meta: Arc<RwLock<ServerMeta>>,
    logger: Logger,
    service: Arc<Mutex<dyn RaftService+Send>>,
}

impl MpcService {
    pub fn new(logger: Logger, id: u64, meta: Arc<RwLock<ServerMeta>>, service: Arc<Mutex<dyn RaftService+Send>>,) -> Self {
        MpcService { logger, id, meta, service }
    }
}

impl Mpc for MpcService {
    fn send(&mut self, ctx: RpcContext, req: Message, sink: UnarySink<MpcReply>) {
        let mut resp = MpcReply::default();
        resp.set_code(MpcReplyCode::Ok);

        if let Ok(mut cmd) = bincode::deserialize::<Command>(&req.get_context()) {
            match cmd.command {
                CommandType::Join => {
                    let leader_id;
                    {
                        let meta_g = self.meta.read().unwrap();
                        leader_id = meta_g.leader_id;
                    }
                    if leader_id != self.id {
                        resp.set_code(MpcReplyCode::ERR);
                        resp.set_msg(format!("The node is not the leader"));
                    } else {
                        cmd.host = Some(get_peer_host(&ctx.peer()));

                        self.service.lock().unwrap().send(Msg::Command(cmd));

                        match bincode::serialize(&leader_id) {
                            Ok(bs) => resp.set_context(bs),
                            Err(e) => {
                                resp.set_code(MpcReplyCode::ERR);
                                error!(self.logger, "Fail to serialize id: {}", e)
                            }
                        }
                    }
                }
            }

        } else {
            self.service.lock().unwrap().send(Msg::Raft(req));
        }

        let logger = self.logger.clone();
        let f = sink.success(resp).map_err(move|e|warn!(logger, "Fail to reply {:?}", e));
        ctx.spawn(f)
    }
}

pub struct MpcServer {
    id: u64,
    port: u16,
    meta: Arc<RwLock<ServerMeta>>,
    channels: usize,
    service: Arc<Mutex<dyn RaftService+Send>>,
    logger: Logger
}

impl MpcServer {
    pub fn new(logger: Logger, id: u64, port: u16, meta: Arc<RwLock<ServerMeta>>, service: Arc<Mutex<dyn RaftService+Send>>, channels: usize) -> Self {
        MpcServer {
            id,
            port,
            meta,
            channels,
            service,
            logger
        }
    }

    pub fn run(&mut self) {
        let env = Arc::new(Environment::new(self.channels));
        let quota = ResourceQuota::new(Some("MpcServer")).resize_memory(1024 * 1024);
        let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);
        let service = create_mpc(MpcService::new(self.logger.clone(), self.id, self.meta.clone(), self.service.clone()));

        let mut server = ServerBuilder::new(env)
            .register_service(service)
            .bind("0.0.0.0", self.port)
            .channel_args(ch_builder.build_args())
            .build()
            .unwrap();

        let logger = self.logger.clone();
        let port = self.port;
        thread::spawn(move || {
            server.start();
            info!(logger, "Start server at {}", port);
            loop {};
        });
    }
}


pub fn create_rpc_client(addr: &String) -> MpcClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&addr);
    let client = MpcClient::new(ch);
    client
}

fn get_peer_host(peer: &String) -> String {
    let splited: Vec<&str> = peer.split(":").collect();
    splited[1].to_string()
}
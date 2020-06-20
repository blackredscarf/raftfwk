#[macro_use]
extern crate serde;
extern crate raftfwk;

use std::time::Duration;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::thread;

use raftfwk::msg::{Msg, RaftError, Normal};
use raftfwk::service::{ProposeCallback, RaftContext, RaftService};
use raftfwk::conf::ConfChange;
use raftfwk::logger::create_logger;
use raftfwk::mem_storage::create_memory_storage;
use raftfwk::{RaftServer, RaftConfig};
use raftfwk::level_storage::create_leveldb_storage;
use raftfwk::storage::RaftStorage;

use rand::{thread_rng, Rng};
use slog::Logger;

use structopt::StructOpt;
use serde::{Deserialize, Serialize};

pub struct KvService {
    callbacks: HashMap<String, ProposeCallback>,
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
    logger: Logger,
    context: RaftContext,
    kv: HashMap<String, Vec<u8>>
}

impl KvService {
    pub fn new(logger: Logger) -> Self {
        let (sender, receiver) = mpsc::channel();
        KvService {
            logger,
            sender,
            receiver,
            context: RaftContext::new(0),
            callbacks: HashMap::new(),
            kv: HashMap::new()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub key: String,
    pub value: Vec<u8>
}

impl RaftService for KvService {
    fn recv(&mut self) -> Result<Msg, TryRecvError> {
        self.receiver.try_recv()
    }

    fn send(&mut self, msg: Msg) {
        self.sender.send(msg);
    }

    fn update_context(&mut self, ctx: RaftContext) {
        self.context = ctx;
    }

    fn context(&mut self) -> RaftContext {
        self.context.clone()
    }

    fn propose(&mut self, cmd: i32, data: &Vec<u8>, callback: Option<ProposeCallback>) -> Result<(), RaftError> {
        if cmd == 0 && !self.context.is_leader() {
            return Err(RaftError::new(1020, format!("The node is not a leader")))
        }

        let mut rng = thread_rng();
        let id = format!("{}", rng.gen::<i32>());

        let proposal = (id.clone(), cmd, data.clone());
        let bytes = bincode::serialize(&proposal).unwrap();

        self.send(Msg::Propose(Normal { data: bytes }));

        match callback {
            Some(cb) => {
                self.callbacks.insert(id, cb);
            },
            None => ()
        }

        Ok(())
    }

    fn propose_conf_change(&mut self, cc: ConfChange) -> Result<(), RaftError> {
        if !self.context.is_leader() {
            return Err(RaftError::new(1020, format!("The node is not a leader")))
        }
        self.sender.send(Msg::ConfChange(cc));
        Ok(())
    }

    fn on_committed(&mut self, data: &Vec<u8>) {
        let proposal: (String, i32, Vec<u8>) = bincode::deserialize(data).unwrap();
        let item: Item = bincode::deserialize(&proposal.2).unwrap();
        let cbo = self.callbacks.get(&proposal.0);
        if proposal.1 == 0 {
            self.kv.insert(item.key, item.value);
            if let Some(cb) = cbo {
                cb(None);
            }
        } else {
            match self.kv.get(&item.key) {
                Some(v) => {
                    if let Some(cb) = cbo {
                        cb(Some(v.clone()));
                    }
                },
                None => {
                    if let Some(cb) = cbo {
                        cb(None);
                    }
                }
            }
        }
    }
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short="i")]
    id: u64,

    #[structopt(short="p")]
    port: u16,

    #[structopt(short="l", default_value="info")]
    log_level: String,

    /// The method of saving logs, memory or leveldb.
    #[structopt(short="s", default_value="memory")]
    storage: String,

    /// Specifying a cluster to join in. Giving the leader url like 168.192.32.108:8010.
    #[structopt(short="c")]
    cluster: Option<String>
}

pub struct KvCli {
    service: Arc<Mutex<KvService>>
}

impl KvCli {
    pub fn new(service: Arc<Mutex<KvService>>) -> Self {
        KvCli { service }
    }

    fn map(&mut self, input: Vec<String>) {
        if &input[0] == "set" {
            self.set(input);
        } else if &input[0] == "get" {
            self.get(input);
        }
    }

    pub fn start(&mut self) {
        loop {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line = line.trim().to_string();
            let splited: Vec<&str> = line.split(" ").collect();
            self.map(splited.iter().map(|x| String::from(*x)).collect());
        }
    }

    fn set(&self, vs: Vec<String>) {
        let (sender, receiver) = mpsc::channel();
        let cb = Box::new(move |x: Option<Vec<u8>>| {
            if let Some(v) = x {
                sender.send(String::from_utf8(v).unwrap());
            } else {
                sender.send(format!("Success"));
            }
        });

        let v = Item {
            key: vs[1].clone(),
            value: vs[2].clone().into_bytes()
        };
        let bytes = bincode::serialize(&v).unwrap();

        match self.service.lock().unwrap().propose(0, &bytes, Some(cb)) {
            Ok(v)=> (),
            Err(e) => println!("Error: {}", e)
        }

        match receiver.recv_timeout(Duration::from_secs(3)) {
            Ok(v) => println!("Set: {}", v),
            Err(e) => println!("Error: {}", e)
        }
    }

    fn get(&self, vs: Vec<String>) {
        let (sender, receiver) = mpsc::channel();
        let cb = Box::new(move |x: Option<Vec<u8>>| {
            sender.send(x);
        });

        let v = Item { key: vs[1].clone(), value: vec![] };
        let bytes = bincode::serialize(&v).unwrap();

        self.service.lock().unwrap().propose(1, &bytes, Some(cb));

        match receiver.recv_timeout(Duration::from_secs(3)) {
            Ok(v) => {
                if let Some(data) = v {
                    println!("Value: {}", String::from_utf8(data).unwrap());
                } else {
                    println!("Warn: Not found data");
                }
            },
            Err(e) => println!("Error: {}", e)
        }
    }

}

pub fn start_cli(mut cli: KvCli) {
    thread::spawn(move || {
        cli.start();
    });
}

fn main() {
    let args: Args = Args::from_args();

    let logger = create_logger(args.id, args.log_level);

    let service = Arc::new(Mutex::new(KvService::new(logger.clone())));

    let mut cli = KvCli::new(service.clone());
    start_cli(cli);

    if &args.storage == "memory" {
        let storage = create_memory_storage();
        let config = RaftConfig::join(args.id, args.port, args.cluster);
        let mut r = RaftServer::new(logger.clone(), config, storage, service.clone());
        r.run();
    } else {
        let storage = create_leveldb_storage(format!("db{}", args.id), logger.clone());
        let config = RaftConfig::join(args.id, args.port, args.cluster);
        let mut r = RaftServer::new(logger.clone(), config, storage, service.clone());
        r.run();
    }
}
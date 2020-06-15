use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::collections::HashMap;
use raft::eraftpb::ConfChange;
use serde::{Serialize, Deserialize};
use crate::msg::*;

pub type ProposeCallback = Box<dyn Fn(Option<Vec<u8>>) + Send>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaftContext {
    pub id: u64,
    pub leader_id: u64,
    pub peers: HashMap<u64, String>
}

impl RaftContext {
    pub fn new(id: u64) -> Self {
        RaftContext {
            id,
            leader_id: 0,
            peers: HashMap::new()
        }
    }

    pub fn is_leader(&self) -> bool {
        self.id == self.leader_id
    }
}

pub trait RaftService {

    fn recv(&mut self) -> Result<Msg, TryRecvError>;

    fn send(&mut self, msg: Msg);

    fn update_context(&mut self, ctx: RaftContext);

    fn context(&mut self) -> RaftContext;

    fn propose(&mut self, cmd: i32, data: &Vec<u8>, callback: Option<ProposeCallback>) -> Result<(), RaftError>;

    fn propose_conf_change(&mut self, cc: ConfChange) -> Result<(), RaftError>;

    fn on_committed(&mut self, data: &Vec<u8>);
}

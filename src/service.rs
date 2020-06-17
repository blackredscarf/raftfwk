use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::collections::HashMap;
use raft::eraftpb::ConfChange;
use serde::{Serialize, Deserialize};
use crate::msg::*;

pub type ProposeCallback = Box<dyn Fn(Option<Vec<u8>>) + Send>;

/// A Object contains status information of raft.
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

/// A trait for user to implement business logic.
pub trait RaftService {

    /// Try to get a message.
    fn recv(&mut self) -> Result<Msg, TryRecvError>;

    /// Send a message.
    fn send(&mut self, msg: Msg);

    /// Update RaftContext in a tick.
    fn update_context(&mut self, ctx: RaftContext);

    /// Get RaftContext.
    fn context(&mut self) -> RaftContext;

    /// Propose.
    ///     - `cmd` is a flag of the proposal.
    ///     - `callback` is a function stored by user itself. You can use it in method `on_committed`.
    fn propose(&mut self, cmd: i32, data: &Vec<u8>, callback: Option<ProposeCallback>) -> Result<(), RaftError>;

    /// Propose a ConfChange.
    fn propose_conf_change(&mut self, cc: ConfChange) -> Result<(), RaftError>;

    /// Calling on committed of a proposal.
    fn on_committed(&mut self, data: &Vec<u8>);
}

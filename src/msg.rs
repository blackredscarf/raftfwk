use raft::eraftpb::Message;
use raft::eraftpb::ConfChange;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

pub enum Msg {
    Propose(Normal),
    Command(Command),
    ConfChange(ConfChange),
    Raft(Message),
}

pub struct Normal {
    pub data: Vec<u8>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Command {
    pub command: CommandType,
    pub id: u64,
    pub port: u16,
    pub host: Option<String>
}

impl Command {
    pub fn new_join(id: u64, port: u16) -> Self {
        Command { id, port, command: CommandType::Join, host: None }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CommandType {
    Join = 0
}

#[derive(Debug)]
pub struct RaftError {
    pub what: i32,
    pub msg: String
}

impl RaftError {
    pub fn new(what: i32, msg: String) -> Self {
        RaftError { what, msg }
    }
}

impl Error for RaftError {}

impl fmt::Display for RaftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RaftError: {}", self.msg)
    }
}
use raft::storage::MemStorage;
use raft::{Result, Error};
use raft::prelude::*;

pub trait RaftStorage: Storage {

    fn initialize_with_conf_state<T>(&self, conf_state: T) where ConfState: From<T>;

    fn set_hardstate(&mut self, hs: HardState);

    fn set_conf_state(&mut self, cs: ConfState);

    fn apply_snapshot(&mut self, snapshot: Snapshot) -> Result<()>;

    fn hard_state(&mut self) -> HardState;

    fn commit_to(&mut self, index: u64) -> Result<()>;

    fn compact(&mut self, compact_index: u64) -> Result<()>;

    fn append(&mut self, ents: &[Entry]) -> Result<()>;
}

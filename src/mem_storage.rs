use raft::storage::MemStorage;
use raft::{Result, Error};
use raft::prelude::*;
use protobuf::Message as GBMessage;
use crate::storage::RaftStorage;

pub struct MemoryStorage {
    m: MemStorage
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            m: MemStorage::new()
        }
    }

    pub fn new_with_conf_state<T>(conf_state: T) -> MemoryStorage
        where
            ConfState: From<T>
    {
        MemoryStorage {
            m: MemStorage::new_with_conf_state(conf_state)
        }
    }
}

impl Storage for MemoryStorage {
    fn initial_state(&self) -> Result<RaftState> {
        self.m.initial_state()
    }

    fn entries(&self, low: u64, high: u64, max_size: impl Into<Option<u64>>) -> Result<Vec<Entry>> {
        self.m.entries(low, high, max_size)
    }

    fn term(&self, idx: u64) -> Result<u64> {
        self.m.term(idx)
    }

    fn first_index(&self) -> Result<u64> {
        self.m.first_index()
    }

    fn last_index(&self) -> Result<u64> {
        self.m.last_index()
    }

    fn snapshot(&self, request_index: u64) -> Result<Snapshot> {
        self.m.snapshot(request_index)
    }
}

impl RaftStorage for MemoryStorage {

    fn initialize_with_conf_state<T>(&self, conf_state: T)
        where
            ConfState: From<T>
    {
        self.m.initialize_with_conf_state(conf_state);
    }

    fn set_hardstate(&mut self, hs: HardState) {
        self.m.wl().set_hardstate(hs);
    }

    fn set_conf_state(
        &mut self,
        cs: ConfState
    ) {
        self.m.wl().set_conf_state(cs);
    }

    fn apply_snapshot(&mut self, snapshot: Snapshot) -> Result<()> {
        self.m.wl().apply_snapshot(snapshot)
    }

    fn hard_state(&mut self) -> HardState {
        self.m.wl().mut_hard_state().clone()
    }

    fn commit_to(&mut self, index: u64) -> Result<()> {
        self.m.wl().commit_to(index)
    }

    fn compact(&mut self, compact_index: u64) -> Result<()> {
        self.m.wl().compact(compact_index)
    }

    fn append(&mut self, ents: &[Entry]) -> Result<()> {
        self.m.wl().append(ents)
    }
}


pub fn create_memory_storage(id: Option<u64>) -> MemoryStorage {
    if let Some(id) = id {
        return MemoryStorage::new_with_conf_state(ConfState::from((vec![id], vec![])))
    }
    MemoryStorage::new()
}


use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::panic;

use rusty_leveldb::{DB, Options};
use serde::{Serialize, Deserialize};
use raft::{Storage, RaftState, StorageError};
use raft::{Result, Error};
use raft::prelude::*;
use raft::util::limit_size;
use protobuf::{Message as GBMessage, UnknownFields, CachedSize};

use crate::storage::RaftStorage;
use slog::Logger;


struct LevelStorageCore {
    db: DB,
    logger: Logger,
    entries: Vec<Entry>,
    raft_state: RaftState,
    snapshot_metadata: SnapshotMetadata
}

impl LevelStorageCore {

    fn new(name: String, logger: Logger) -> LevelStorageCore {
        let mut opt = Options::default();
        opt.create_if_missing = true;
        let mut db = DB::open(name, opt).unwrap();

        let mut raft_state = RaftState::default();
        let mut snapshot_metadata = SnapshotMetadata::default();

        // Snapshot
        let mut snap = Snapshot::new();
        if let Some(b) = db.get(b"snapshot") {
            info!(logger, "Load snapshot");
            snap.merge_from_bytes(&b).unwrap();
            snapshot_metadata = snap.take_metadata().clone();
            raft_state.hard_state.term = snapshot_metadata.term;
            raft_state.hard_state.commit = snapshot_metadata.index;
            raft_state.conf_state = snapshot_metadata.take_conf_state();
        }

        info!(logger, "Start node at term: {}, index: {}", raft_state.hard_state.term, raft_state.hard_state.commit);

        // Entries
        let mut entries = vec![];

        let mut i = raft_state.hard_state.commit + 1;
        loop {
            let k = format!("entries{}", i);
            i += 1;
            let mut entry = Entry::new();
            if let Some(b) = db.get(&k.clone().into_bytes()) {
                entry.merge_from_bytes(&b).unwrap();
                entries.push(entry);
            } else {
                break;
            }
        }
        debug!(logger, "Load number of entry: {}", entries.len());

        LevelStorageCore {
            db,
            logger,
            entries,
            raft_state,
            snapshot_metadata,
        }
    }

    /// Saves the current HARD_STATE.
    pub fn set_hardstate(&mut self, hs: HardState) {
        self.raft_state.hard_state = hs;
    }

    /// Get the hard state.
    pub fn hard_state(&self) -> &HardState {
        &self.raft_state.hard_state
    }

    /// Get the mut hard state.
    pub fn mut_hard_state(&mut self) -> &mut HardState {
        &mut self.raft_state.hard_state
    }

    /// Commit to an index.
    ///
    /// # Panics
    ///
    /// Panics if there is no such entry in raft logs.
    pub fn commit_to(&mut self, index: u64) -> Result<()> {
        assert!(
            self.has_entry_at(index),
            "commit_to {} but the entry not exists",
            index
        );

        let diff = (index - self.entries[0].index) as usize;
        self.raft_state.hard_state.commit = index;
        self.raft_state.hard_state.term = self.entries[diff].term;
        Ok(())
    }

    /// Saves the current conf state.
    pub fn set_conf_state(&mut self, cs: ConfState) {
        self.raft_state.conf_state = cs;
    }

    #[inline]
    fn has_entry_at(&self, index: u64) -> bool {
        !self.entries.is_empty() && index >= self.first_index() && index <= self.last_index()
    }

    fn first_index(&self) -> u64 {
        match self.entries.first() {
            Some(e) => e.index,
            None => self.snapshot_metadata.index + 1,
        }
    }

    fn last_index(&self) -> u64 {
        match self.entries.last() {
            Some(e) => e.index,
            None => self.snapshot_metadata.index,
        }
    }

    /// Overwrites the contents of this Storage object with those of the given snapshot.
    ///
    /// # Panics
    ///
    /// Panics if the snapshot index is less than the storage's first index.
    pub fn apply_snapshot(&mut self, mut snapshot: Snapshot) -> Result<()> {

        self.db.put(b"snapshot", &snapshot.write_to_bytes().unwrap());
        self.db.flush().unwrap();

        let mut meta = snapshot.take_metadata();
        let term = meta.term;
        let index = meta.index;

        if self.first_index() > index {
            return Err(Error::Store(StorageError::SnapshotOutOfDate));
        }

        self.snapshot_metadata = meta.clone();

        self.raft_state.hard_state.term = term;
        self.raft_state.hard_state.commit = index;
        self.entries.clear();

        // Update conf states.
        self.raft_state.conf_state = meta.take_conf_state();

        Ok(())
    }

    fn snapshot(&self) -> Snapshot {
        let mut snapshot = Snapshot::default();

        // Use the latest applied_idx to construct the snapshot.
        let applied_idx = self.raft_state.hard_state.commit;
        let term = self.raft_state.hard_state.term;
        let mut meta = snapshot.mut_metadata();
        meta.index = applied_idx;
        meta.term = term;

        meta.set_conf_state(self.raft_state.conf_state.clone());
        snapshot
    }

    /// Discards all log entries prior to compact_index.
    /// It is the application's responsibility to not attempt to compact an index
    /// greater than RaftLog.applied.
    ///
    /// # Panics
    ///
    /// Panics if `compact_index` is higher than `Storage::last_index(&self) + 1`.
    pub fn compact(&mut self, compact_index: u64) -> Result<()> {
        if compact_index <= self.first_index() {
            // Don't need to treat this case as an error.
            return Ok(());
        }

        if compact_index > self.last_index() + 1 {
            panic!(
                "compact not received raft logs: {}, last index: {}",
                compact_index,
                self.last_index()
            );
        }

        if let Some(entry) = self.entries.first() {
            let offset = compact_index - entry.index;
            self.entries.drain(..offset as usize);
        }

        Ok(())
    }

    /// Append the new entries to storage.
    ///
    /// # Panics
    ///
    /// Panics if `ents` contains compacted entries, or there's a gap between `ents` and the last
    /// received entry in the storage.
    pub fn append(&mut self, ents: &[Entry]) -> Result<()> {
        if ents.is_empty() {
            return Ok(());
        }
        if self.first_index() > ents[0].index {
            panic!(
                "overwrite compacted raft logs, compacted: {}, append: {}",
                self.first_index() - 1,
                ents[0].index,
            );
        }
        if self.last_index() + 1 < ents[0].index {
            panic!(
                "raft logs should be continuous, last index: {}, new appended: {}",
                self.last_index(),
                ents[0].index,
            );
        }

        // Remove all entries overwritten by `ents`.
        let diff = ents[0].index - self.first_index();
        self.entries.drain(diff as usize..);
        self.entries.extend_from_slice(&ents);

        for ent in ents {
            let name = format!("entries{}", ent.index);
            self.db.put(&name.clone().into_bytes(), &ent.write_to_bytes().unwrap());
        }
        self.db.flush().unwrap();

        Ok(())
    }

    /// Commit to `idx` and set configuration to the given states. Only used for tests.
    pub fn commit_to_and_set_conf_states(&mut self, idx: u64, cs: Option<ConfState>) -> Result<()> {
        self.commit_to(idx)?;
        if let Some(cs) = cs {
            self.raft_state.conf_state = cs;
        }
        Ok(())
    }

}

pub struct LevelStorage {
    core: Arc<RwLock<LevelStorageCore>>
}


impl LevelStorage {
    pub fn new(name: String, logger: Logger) -> LevelStorage {
        LevelStorage {
            core: Arc::new(RwLock::new(LevelStorageCore::new(name, logger)))
        }
    }

    pub fn initialize_with_conf_state<T>(&self, conf_state: T) where ConfState: From<T>, {
        assert!(!self.initial_state().unwrap().initialized());
        let mut core = self.wl();
        core.raft_state.conf_state = ConfState::from(conf_state);
    }

    fn rl(&self) -> RwLockReadGuard<LevelStorageCore> {
        self.core.read().unwrap()
    }

    fn wl(&self) -> RwLockWriteGuard<LevelStorageCore> {
        self.core.write().unwrap()
    }
}


impl Storage for LevelStorage {
    fn initial_state(&self) -> Result<RaftState> {
        Ok(self.rl().raft_state.clone())
    }

    fn entries(&self, low: u64, high: u64, max_size: impl Into<Option<u64>>) -> Result<Vec<Entry>> {
        let max_size = max_size.into();
        let core = self.rl();
        if low < core.first_index() {
            return Err(Error::Store(StorageError::Compacted));
        }

        if high > core.last_index() + 1 {
            panic!(
                "index out of bound (last: {}, high: {})",
                core.last_index() + 1,
                high
            );
        }

        let offset = core.entries[0].index;
        let lo = (low - offset) as usize;
        let hi = (high - offset) as usize;
        let mut ents = core.entries[lo..hi].to_vec();
        limit_size(&mut ents, max_size);
        Ok(ents)
    }

    fn term(&self, idx: u64) -> Result<u64> {
        let core = self.rl();
        if idx == core.snapshot_metadata.index {
            return Ok(core.snapshot_metadata.term);
        }

        if idx < core.first_index() {
            return Err(Error::Store(StorageError::Compacted));
        }

        let offset = core.entries[0].index;
        assert!(idx >= offset);
        if idx - offset >= core.entries.len() as u64 {
            return Err(Error::Store(StorageError::Unavailable));
        }
        Ok(core.entries[(idx - offset) as usize].term)
    }

    fn first_index(&self) -> Result<u64> {
        Ok(self.rl().first_index())
    }

    fn last_index(&self) -> Result<u64> {
        Ok(self.rl().last_index())
    }

    fn snapshot(&self, request_index: u64) -> Result<Snapshot> {
        let mut core = self.wl();
        let mut snap = core.snapshot();
        if snap.get_metadata().index < request_index {
            snap.mut_metadata().index = request_index;
        }
        Ok(snap)
    }
}

impl RaftStorage for LevelStorage {
    fn initialize_with_conf_state<T>(&self, conf_state: T) where ConfState: From<T> {
        self.wl().raft_state.conf_state = ConfState::from(conf_state);
    }

    fn set_hardstate(&mut self, hs: HardState) {
        self.wl().set_hardstate(hs);
    }

    fn set_conf_state(&mut self, cs: ConfState) {
        self.wl().set_conf_state(cs);
    }

    fn apply_snapshot(&mut self, snapshot: Snapshot) -> Result<()> {
        self.wl().apply_snapshot(snapshot)
    }

    fn hard_state(&self) -> HardState {
        self.wl().mut_hard_state().clone()
    }

    fn commit_to(&mut self, index: u64) -> Result<()> {
        self.wl().commit_to(index)
    }

    fn compact(&mut self, compact_index: u64) -> Result<()> {
        self.wl().compact(compact_index)
    }

    fn append(&mut self, ents: &[Entry]) -> Result<()> {
        self.wl().append(ents)
    }
}

pub fn create_leveldb_storage(db_name: String, logger: Logger) -> LevelStorage {
    LevelStorage::new(db_name, logger)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_storage() {

    }
}
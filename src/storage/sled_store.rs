use crate::record::Record;
use core::convert::AsRef;
use sled::Db;
use std::path::Path;
use super::record_store::RecordStore;

pub struct SledStore {
    db: Db,
}

impl SledStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let db: Db = sled::open(path).unwrap();
        Self { db }
    }
}

impl RecordStore for SledStore {
    fn look_up(&self, name: &str) -> Record {
      Record::new()
    }
    fn insert(&self, record: &Record) {}
}

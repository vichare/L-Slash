use super::record_store::RecordStore;
use crate::record::Record;

use core::convert::AsRef;
use protobuf::Message;
use sled::Db;
use std::error::Error;
use std::ops::RangeBounds;
use std::path::Path;

pub struct SledStore {
    db: Db,
}

impl SledStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, sled::Error> {
        let db: Db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn look_up_protobuf<ProtoType: Message>(
        tree: &sled::Tree,
        name: &str,
    ) -> Result<Option<ProtoType>, Box<dyn Error>> {
        let maybe_buffer = tree.get(name)?;
        maybe_buffer
            .map(|buffer| ProtoType::parse_from_bytes(buffer.as_ref()))
            .transpose()
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub fn look_up_protobuf_default<ProtoType: Message>(
        &self,
        name: &str,
    ) -> Result<Option<ProtoType>, Box<dyn Error>> {
        Self::look_up_protobuf(&self.db, name)
    }

    pub fn insert_protobuf<Key: AsRef<[u8]>, ProtoType: Message>(
        tree: &sled::Tree,
        name: Key,
        record: &ProtoType,
    ) -> Result<(), Box<dyn Error>> {
        let bytes = record.write_to_bytes()?;
        tree.insert(name, bytes)?;
        Ok(())
    }

    pub fn insert_protobuf_default<Key: AsRef<[u8]>, ProtoType: Message>(
        &self,
        name: Key,
        record: &ProtoType,
    ) -> Result<(), Box<dyn Error>> {
        Self::insert_protobuf(&self.db, name, record)
    }

    pub fn list<ProtoType: Message>(
        &self,
        range: impl RangeBounds<String>,
    ) -> impl Iterator<Item = Result<ProtoType, Box<dyn Error>>> {
        self.db.range(range).map(|result_ivec| {
            let (_key, value) = result_ivec?;
            ProtoType::parse_from_bytes(value.as_ref()).map_err(|e| Box::new(e) as Box<dyn Error>)
        })
    }

    /*
    pub fn look_up_protobuf_default<ProtoType: Message>(
        &self,
        name: &str,
    ) -> Result<Option<ProtoType>, Box<dyn Error>> {
        let maybe_buffer = self.db.get(name)?;
        maybe_buffer
            .map(|buffer| ProtoType::parse_from_bytes(buffer.as_ref()))
            .transpose()
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
    pub fn insert_protobuf_default<Key: AsRef<[u8]>, ProtoType: Message>(
        &self,
        name: Key,
        record: &ProtoType,
    ) -> Result<(), Box<dyn Error>> {
        let bytes = record.write_to_bytes()?;
        self.db.insert(name, bytes)?;
        Ok(())
    }
    fn into_protobuf<ErrorType: std::error::Error, ProtoType: Message>(
        result: Result<Option<sled::IVec>, ErrorType>,
    ) -> Result<Option<ProtoType>, Box<dyn Error>> {
        let maybe_buffer = result?;
        maybe_buffer
            .map(|buffer| ProtoType::parse_from_bytes(buffer.as_ref()))
            .transpose()
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
    */
}

impl RecordStore for SledStore {
    fn look_up(&self, name: &str) -> Result<Option<Record>, Box<dyn Error>> {
        Self::look_up_protobuf(&self.db, name)
    }

    fn insert(&self, record: &Record) -> Result<(), Box<dyn Error>> {
        Self::insert_protobuf(&self.db, record.name(), record)
    }
}

use super::record_store::RecordStore;
use crate::record::Record;
use crate::session::Session;
use crate::user::User;

use core::convert::AsRef;
use protobuf::Message;
use sled::Db;
use std::error::Error;
use std::ops::RangeBounds;
use std::path::Path;

const SESSION_TREE_NAME: &[u8] = b"session";
const USER_TREE_NAME: &[u8] = b"user";

pub struct SledStore {
    db: Db,
}

impl SledStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, sled::Error> {
        let db: Db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn look_up_user(&self, username: &str) -> Result<Option<User>, Box<dyn Error>> {
        Self::look_up_protobuf(&(self.db.open_tree(USER_TREE_NAME)?), username)
    }

    pub fn insert_user(&self, user: &User) -> Result<(), Box<dyn Error>> {
        Self::insert_protobuf(
            &(self.db.open_tree(USER_TREE_NAME)?),
            user.user_name(),
            user,
        )
    }

    pub fn look_up_session(&self, session_key: &str) -> Result<Option<Session>, Box<dyn Error>> {
        Self::look_up_protobuf(&(self.db.open_tree(SESSION_TREE_NAME)?), session_key)
    }

    pub fn insert_session(&self, session: &Session) -> Result<(), Box<dyn Error>> {
        Self::insert_protobuf(
            &(self.db.open_tree(SESSION_TREE_NAME)?),
            session.key(),
            session,
        )
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

    pub fn insert_protobuf<Key: AsRef<[u8]>, ProtoType: Message>(
        tree: &sled::Tree,
        name: Key,
        record: &ProtoType,
    ) -> Result<(), Box<dyn Error>> {
        let bytes = record.write_to_bytes()?;
        tree.insert(name, bytes)?;
        Ok(())
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
}

impl RecordStore for SledStore {
    fn look_up(&self, name: &str) -> Result<Option<Record>, Box<dyn Error>> {
        Self::look_up_protobuf(&self.db, name)
    }

    fn insert(&self, record: &Record) -> Result<(), Box<dyn Error>> {
        Self::insert_protobuf(&self.db, record.name(), record)
    }
}

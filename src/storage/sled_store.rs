// use crate::Record;
use crate::Record;
use crate::Session;
use crate::User;

use core::convert::AsRef;
use sled::Db;
use sled::Tree;
use std::marker::PhantomData;
use std::ops::RangeBounds;
use std::path::Path;

const USER_TREE_NAME: &[u8] = b"user";
const SESSION_TREE_NAME: &[u8] = b"session";
const RECORD_TREE_NAME: &[u8] = b"record";

#[derive(thiserror::Error, Debug)]
pub enum DataStoreError {
    #[error("Sled error: {0}")]
    Sled(#[from] sled::Error),
    #[error("Decode error: {0}")]
    Decode(#[from] ::protobuf::ParseError),
    #[error("Encode error: {0}")]
    Encode(#[from] ::protobuf::SerializeError),
    #[error("Unknown error: {0}")]
    Unknown(#[from] std::io::Error),
}

pub trait DataType: Default + Clone + Send + Sync + Sized + ::protobuf::Message {
    // fn parse_from_bytes(buffer: &[u8]) -> Result<Self, DataStoreError> {
    //     Self::parse(buffer).map_err(|e| DataStoreError::Decode(e))
    // }
    //
    // fn to_bytes(&self) -> Result<Vec<u8>, DataStoreError> {
    //     return self.serialize().map_err(|e| DataStoreError::Encode(e));
    // }
}

impl<T: Default + Clone + Send + Sync + Sized + ::protobuf::Message> DataType for T {}

pub struct SledTree<D: DataType> {
    // store: SledStore,
    pub tree: Tree,
    phantom: PhantomData<D>,
}

impl<D: DataType> SledTree<D> {
    pub fn new<Key: AsRef<[u8]>>(db: Db, name: Key) -> Result<Self, DataStoreError> {
        let tree: Tree = db.open_tree(name)?;
        Ok(Self {
            tree,
            phantom: PhantomData,
        })
    }

    pub fn look_up<Key: AsRef<[u8]>>(&self, key: Key) -> Result<Option<D>, DataStoreError> {
        // let maybe_buffer = self.tree.get(key)?;
        // let buffer = match maybe_buffer {
        //     Some(buf) => buf,
        //     None => return Ok(None),
        // };
        // D::parse(buffer.as_ref())
        //     .map(Some)
        //     .map_err(DataStoreError::Decode)

        self.tree
            .get(key)?
            .map(|buffer| D::parse(buffer.as_ref()).map_err(DataStoreError::Decode))
            .transpose()
    }

    pub fn insert<K: AsRef<[u8]>>(&self, key: K, value: &D) -> Result<(), DataStoreError> {
        let bytes = value.serialize()?;
        self.tree.insert(key, bytes)?;
        Ok(())
    }

    pub fn remove<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<D>, DataStoreError> {
        let maybe_bytes = self.tree.remove(key)?;
        maybe_bytes
            .map(|bytes| D::parse(bytes.as_ref()).map_err(DataStoreError::Decode))
            .transpose()
    }

    pub fn range(
        &self,
        range: impl RangeBounds<String>,
    ) -> impl Iterator<Item = Result<D, DataStoreError>> {
        self.tree.range(range).values().map(|result| {
            result
                .map_err(DataStoreError::Sled)
                .and_then(|bytes| D::parse(bytes.as_ref()).map_err(DataStoreError::Decode))
        })
    }
}

pub struct SledStore {
    pub db: Db,
    pub users: SledTree<User>,
    pub sessions: SledTree<Session>,
    pub records: SledTree<Record>,
}

impl SledStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DataStoreError> {
        let db: Db = sled::open(path)?;
        let users = SledTree::new(db.clone(), USER_TREE_NAME)?;
        let sessions = SledTree::new(db.clone(), SESSION_TREE_NAME)?;
        let records = SledTree::new(db.clone(), RECORD_TREE_NAME)?;
        Ok(Self {
            db,
            users,
            sessions,
            records,
        })
    }

    pub fn look_up_user(&self, username: &str) -> Result<Option<User>, DataStoreError> {
        // Self::look_up_protobuf(&(self.db.open_tree(USER_TREE_NAME)?), username)
        self.users.look_up(username)
    }

    pub fn insert_user(&self, user: &User) -> Result<(), DataStoreError> {
        // Self::insert_protobuf(
        //     &(self.db.open_tree(USER_TREE_NAME)?),
        //     user.user_name(),
        //     user,
        // )
        self.users.insert(user.user_name(), user)
    }

    pub fn look_up_session(&self, session_key: &[u8]) -> Result<Option<Session>, DataStoreError> {
        self.sessions.look_up(session_key)
    }

    pub fn insert_session(&self, session: &Session) -> Result<(), DataStoreError> {
        self.sessions.insert(session.key(), session)
    }

    // pub fn look_up_protobuf<Key: AsRef<[u8]>, ProtoType: Message>(
    //     tree: &sled::Tree,
    //     name: Key,
    // ) -> Result<Option<ProtoType>, Box<dyn Error>> {
    //     let maybe_buffer = tree.get(name)?;
    //     maybe_buffer
    //         .map(|buffer| ProtoType::parse_from_bytes(buffer.as_ref()))
    //         .transpose()
    //         .map_err(|e| Box::new(e) as Box<dyn Error>)
    // }

    // pub fn insert_protobuf<Key: AsRef<[u8]>, ProtoType: Message>(
    //     tree: &sled::Tree,
    //     name: Key,
    //     record: &ProtoType,
    // ) -> Result<(), Box<dyn Error>> {
    //     let bytes = record.write_to_bytes()?;
    //     tree.insert(name, bytes)?;
    //     Ok(())
    // }

    // pub fn list<ProtoType: Message>(
    //     &self,
    //     range: impl RangeBounds<String>,
    // ) -> impl Iterator<Item = Result<ProtoType, Box<dyn Error>>> {
    //     self.db.range(range).map(|result_ivec| {
    //         let (_key, value) = result_ivec?;
    //         ProtoType::parse_from_bytes(value.as_ref()).map_err(|e| Box::new(e) as Box<dyn Error>)
    //     })
    // }
}

// impl RecordStore for SledStore {
//     fn look_up(&self, name: &str) -> Result<Option<Record>, Box<dyn Error>> {
//         Self::look_up_protobuf(&self.db, name)
//     }
//
//     fn insert(&self, record: &Record) -> Result<(), Box<dyn Error>> {
//         Self::insert_protobuf(&self.db, record.name(), record)
//     }
// }

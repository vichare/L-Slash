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

pub trait DataType: Default + Clone + Send + Sync + Sized + ::protobuf::Message {}
impl<T: Default + Clone + Send + Sync + Sized + ::protobuf::Message> DataType for T {}

#[derive(Clone)]
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

#[derive(Clone)]
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

    pub fn insert_user(&self, user: &User) -> Result<(), DataStoreError> {
        self.users.insert(user.user_name(), user)
    }

    pub fn insert_session(&self, session: &Session) -> Result<(), DataStoreError> {
        self.sessions.insert(session.key(), session)
    }
}

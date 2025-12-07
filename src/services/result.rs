#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Data store error: {0}")]
    DataStore(#[from] crate::storage::sled_store::DataStoreError),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ServiceError>;

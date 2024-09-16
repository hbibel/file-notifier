use super::AccessKey;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("An access key with ID '{0}' already exists")]
    DuplicateId(String),
}

pub trait Store {
    fn store_access_key(&self, key: &AccessKey) -> Result<(), StorageError>;
}

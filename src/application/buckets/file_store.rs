use super::model::{FileData, FileId, Timestamp};

use thiserror::Error;

pub trait FileStore {
    async fn persist<T: FileData>(
        &self,
        file_data: &T,
        file_id: &FileId,
        timestamp: &Timestamp,
    ) -> Result<(), PersistFileError>;

    async fn fetch<T: FileData>(
        &self,
        file_id: &str,
        timestamp: &Timestamp,
    ) -> Result<T, PersistFileError>;
}

#[derive(Error, Debug)]
pub enum PersistFileError {
    #[error("File data could not be persisted: {0}")]
    Unknown(#[source] Box<dyn std::error::Error>),
}

use super::model::{FileIO, FileId};

use thiserror::Error;

pub trait FileStore {
    async fn persist<FileIOType: FileIO>(
        &self,
        file_data: &mut FileIOType,
        file_id: &FileId,
    ) -> Result<(), PersistFileError>;

    // async fn fetch<T: FileData>(&self, file_id: &str) -> Result<T, FetchFileError>;
}

#[derive(Error, Debug)]
pub enum PersistFileError {
    #[error("File data could not be persisted: {0}")]
    Unknown(#[source] Box<dyn std::error::Error>),
}

#[derive(Error, Debug)]
pub enum FetchFileError {
    #[error("File data could not be read: {0}")]
    Unknown(#[source] Box<dyn std::error::Error>),
}

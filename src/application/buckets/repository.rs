use super::model::{BucketId, FileId, FileMeta};
use thiserror::Error;

use crate::application::clients::model::ClientId;

pub trait Repository {
    async fn add(&self, name: &str, owner_id: ClientId) -> Result<BucketId, AddBucketError>;

    async fn add_file(
        &self,
        bucket_name: &str,
        file_meta_data: &FileMeta,
    ) -> Result<(), AddFileError>;

    async fn update_file(&self, file_meta_data: &FileMeta) -> Result<(), UpdateFileMetadataError>;

    async fn delete_file(&self, file_id: &FileId) -> Result<(), DeleteFileError>;

    async fn get_files(&self, bucket_name: &str) -> Result<Vec<FileMeta>, GetFilesError>;

    async fn has_access(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
    ) -> Result<bool, HasAccessError>;
}

#[derive(Error, Debug)]
pub enum AddBucketError {}

#[derive(Error, Debug)]
pub enum AddFileError {
    #[error("No such bucket named {0}")]
    NoSuchBucket(String),
}

#[derive(Error, Debug)]
pub enum UpdateFileMetadataError {
    #[error("No such file with ID '{0}'")]
    NoSuchFile(FileId),
}

#[derive(Error, Debug)]
pub enum GetFilesError {
    #[error("No such bucket named {0}")]
    NoSuchBucket(String),
}

#[derive(Error, Debug)]
pub enum HasAccessError {
    #[error("No such bucket named {0}")]
    NoSuchBucket(String),
}

#[derive(Error, Debug)]
pub enum DeleteFileError {}

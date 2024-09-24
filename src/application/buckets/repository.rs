use super::model::{BucketId, FileId, FileMeta, Timestamp};
use thiserror::Error;

use crate::application::clients::model::ClientId;

pub trait Repository {
    async fn add(&self, name: &str, owner_id: ClientId) -> Result<BucketId, AddBucketError>;

    async fn add_file(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
        file_meta_data: &FileMeta,
        timestamp: &Timestamp,
    ) -> Result<(), AddFileError>;

    async fn update_file(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
        file_meta_data: &FileMeta,
        timestamp: &Timestamp,
    ) -> Result<(), UpdateFileMetadataError>;

    async fn delete_file(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
        file_id: &FileId,
    ) -> Result<(), DeleteFileError>;

    async fn get_files(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
    ) -> Result<Vec<FileMeta>, GetFilesError>;

    async fn has_access(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
    ) -> Result<bool, HasAccessError>;
}

#[derive(Error, Debug)]
pub enum AddBucketError {}

#[derive(Error, Debug)]
pub enum AddFileError {}

#[derive(Error, Debug)]
pub enum UpdateFileMetadataError {}

#[derive(Error, Debug)]
pub enum GetFilesError {}

#[derive(Error, Debug)]
pub enum HasAccessError {}

#[derive(Error, Debug)]
pub enum DeleteFileError {}

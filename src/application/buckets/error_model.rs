#![allow(clippy::enum_variant_names)]

use super::file_store;
use super::repository;
use thiserror::Error;

use crate::application::clients::model::ClientId;

#[derive(Error, Debug)]
pub enum CreateBucketError {
    #[error("Bucket already exists for client")]
    DuplicateBucket,

    #[error(transparent)]
    AddBucketError(#[from] repository::AddBucketError),
}

#[derive(Error, Debug)]
pub enum InsertFileError {
    #[error(transparent)]
    PersistFileError(#[from] file_store::PersistFileError),

    #[error(transparent)]
    AddFileError(#[from] repository::AddFileError),

    #[error(transparent)]
    CheckAccessError(#[from] CheckAccessError),
}

#[derive(Error, Debug)]
pub enum UpdateFileMetadataError {
    #[error(transparent)]
    RepositoryError(#[from] repository::UpdateFileMetadataError),

    #[error(transparent)]
    CheckAccessError(#[from] CheckAccessError),
}

#[derive(Error, Debug)]
pub enum UpdateFileError {
    #[error(transparent)]
    PersistFileError(#[from] file_store::PersistFileError),

    #[error(transparent)]
    UpdateFileMetadataError(#[from] repository::UpdateFileMetadataError),

    #[error(transparent)]
    CheckAccessError(#[from] CheckAccessError),
}

#[derive(Error, Debug)]
pub enum FetchFileError {}

#[derive(Error, Debug)]
pub enum CheckAccessError {
    #[error("NoAccessError: {client_id} does not have access to {bucket_name}")]
    NoAccess {
        client_id: ClientId,
        bucket_name: String,
    },

    #[error(transparent)]
    CouldNotCheck(#[from] repository::HasAccessError),
}

#[derive(Error, Debug)]
pub enum GetFilesError {
    #[error(transparent)]
    RepositoryError(#[from] repository::GetFilesError),

    #[error(transparent)]
    CheckAccessError(#[from] CheckAccessError),
}

#[derive(Error, Debug)]
pub enum DeleteFileError {
    #[error(transparent)]
    RepositoryError(#[from] repository::DeleteFileError),

    #[error(transparent)]
    CheckAccessError(#[from] CheckAccessError),
}

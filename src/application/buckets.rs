#![allow(dead_code)]
// TODO

pub mod error_model;
pub mod file_store;
pub mod model;
pub mod repository;

use super::clients::model::ClientId;
use error_model::{
    CheckAccessError, CreateBucketError, DeleteFileError, GetFilesError, InsertFileError,
    UpdateFileError, UpdateFileMetadataError,
};
use futures::TryFutureExt;
use model::{BucketId, FileIO, FileId, FileMeta};

pub struct BucketService<A: repository::Repository, B: file_store::FileStore> {
    repository: A,
    file_store: B,
}

impl<A: repository::Repository, B: file_store::FileStore> BucketService<A, B> {
    pub async fn create_bucket(
        &self,
        name: &str,
        owner_id: ClientId,
    ) -> Result<BucketId, CreateBucketError> {
        // TODO start transaction?
        let id = self
            .repository
            .add(name, owner_id)
            .await
            .map_err(CreateBucketError::from)?;
        // commit transaction
        Ok(id)
    }

    pub async fn insert_file<FileDataT: FileIO>(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
        file_name: &str,
        file: &mut model::File<FileDataT>,
    ) -> Result<(), InsertFileError> {
        self.check_bucket_access(bucket_name, owner_id).await?;

        let file_id = FileId::new(uuid::Uuid::now_v7().to_string());

        let file_meta_data = FileMeta {
            id: file_id,
            name: file_name.to_owned(),
        };

        // TODO
        // start transaction, commit only if all futures complete
        futures::future::try_join(
            self.repository
                .add_file(bucket_name, &file_meta_data)
                .map_err(InsertFileError::from),
            self.file_store
                .persist(&mut file.data, &file_meta_data.id)
                .map_err(InsertFileError::from),
        )
        .await?;

        Ok(())
    }

    pub async fn update_file<FileDataT: FileIO>(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
        file: &mut model::File<FileDataT>,
    ) -> Result<(), UpdateFileError> {
        self.check_bucket_access(bucket_name, owner_id).await?;

        // TODO
        // start transaction, commit only if all futures complete
        futures::future::try_join(
            self.repository
                .update_file(&file.meta_data)
                .map_err(UpdateFileError::from),
            self.file_store
                .persist(&mut file.data, &file.meta_data.id)
                .map_err(UpdateFileError::from),
        )
        .await?;

        Ok(())
    }

    pub async fn update_file_metadata(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
        file_meta_data: &FileMeta,
    ) -> Result<(), UpdateFileMetadataError> {
        self.check_bucket_access(bucket_name, owner_id).await?;

        self.repository.update_file(file_meta_data).await?;

        Ok(())
    }

    pub async fn delete_file(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
        file_id: &FileId,
    ) -> Result<(), DeleteFileError> {
        self.check_bucket_access(bucket_name, owner_id).await?;

        self.repository.delete_file(file_id).await?;

        Ok(())
    }

    pub async fn get_files(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
    ) -> Result<Vec<FileMeta>, GetFilesError> {
        self.check_bucket_access(bucket_name, owner_id).await?;

        self.repository
            .get_files(bucket_name)
            .await
            .map_err(GetFilesError::RepositoryError)
    }

    async fn check_bucket_access(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
    ) -> Result<(), CheckAccessError> {
        let has_access = self.repository.has_access(bucket_name, owner_id).await?;

        if has_access {
            Err(CheckAccessError::NoAccess {
                client_id: owner_id.to_owned(),
                bucket_name: bucket_name.to_owned(),
            })
        } else {
            Ok(())
        }
    }
}

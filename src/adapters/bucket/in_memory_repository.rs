use crate::application::buckets;
use crate::application::buckets::model::{Bucket, BucketId, FileId, FileMeta};
use crate::application::clients::model::ClientId;

pub struct InMemoryRepository {
    pub buckets: std::cell::RefCell<Vec<Bucket>>,
    pub files: std::cell::RefCell<Vec<(FileMeta, BucketId)>>,
}

impl buckets::repository::Repository for InMemoryRepository {
    async fn add(
        &self,
        name: &str,
        owner_id: ClientId,
    ) -> Result<BucketId, buckets::repository::AddBucketError> {
        let mut buckets = self.buckets.borrow_mut();

        #[allow(clippy::cast_possible_truncation)]
        let id = BucketId(buckets.len() as u32);
        let new_bucket = Bucket {
            id: id.clone(),
            name: name.to_owned(),
            owner_id: owner_id.clone(),
        };

        buckets.push(new_bucket);

        Ok(id)
    }

    async fn add_file(
        &self,
        bucket_name: &str,
        file_meta_data: &FileMeta,
    ) -> Result<(), buckets::repository::AddFileError> {
        let bucket_id = self.bucket_id_of(bucket_name).ok_or(
            buckets::repository::AddFileError::NoSuchBucket(bucket_name.to_string()),
        )?;

        let mut files = self.files.borrow_mut();

        files.push((file_meta_data.to_owned(), bucket_id));

        todo!()
    }

    async fn update_file(
        &self,
        file_meta_data: &FileMeta,
    ) -> Result<(), buckets::repository::UpdateFileMetadataError> {
        let mut files = self.files.borrow_mut();

        let file_id = &file_meta_data.id;
        let idx = files
            .iter()
            .enumerate()
            .find_map(|(i, f)| (&f.0.id == file_id).then_some(i))
            .ok_or(buckets::repository::UpdateFileMetadataError::NoSuchFile(
                file_id.clone(),
            ))?;
        let bucket_id = files[idx].1.clone();
        let _ = std::mem::replace(&mut files[idx], (file_meta_data.clone(), bucket_id));

        Ok(())
    }

    async fn delete_file(
        &self,
        file_id: &FileId,
    ) -> Result<(), buckets::repository::DeleteFileError> {
        let mut files = self.files.borrow_mut();

        let idx = files
            .iter()
            .enumerate()
            .find_map(|(i, f)| (&f.0.id == file_id).then_some(i));
        if let Some(i) = idx {
            files.remove(i);
        }

        Ok(())
    }

    async fn get_files(
        &self,
        bucket_name: &str,
    ) -> Result<Vec<FileMeta>, buckets::repository::GetFilesError> {
        let bucket_id = self.bucket_id_of(bucket_name).ok_or(
            buckets::repository::GetFilesError::NoSuchBucket(bucket_name.to_string()),
        )?;

        let files = self.files.borrow();

        let files_for_bucket = files
            .iter()
            .filter_map(|f| (f.1 == bucket_id).then_some(f.0.clone()))
            .collect::<Vec<_>>();

        Ok(files_for_bucket)
    }

    async fn has_access(
        &self,
        bucket_name: &str,
        owner_id: &ClientId,
    ) -> Result<bool, buckets::repository::HasAccessError> {
        let buckets = self.buckets.borrow();

        let bucket = buckets.iter().find(|b| b.name == bucket_name).ok_or(
            buckets::repository::HasAccessError::NoSuchBucket(bucket_name.to_string()),
        );

        bucket.map(|b| &b.owner_id == owner_id)
    }
}

impl InMemoryRepository {
    fn bucket_id_of(&self, bucket_name: &str) -> Option<BucketId> {
        let buckets = self.buckets.borrow();
        buckets
            .iter()
            .find_map(|b| (b.name == bucket_name).then_some(b.id.clone()))
    }
}

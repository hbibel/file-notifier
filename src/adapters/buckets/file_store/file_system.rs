use std::path::PathBuf;

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::application::buckets::file_store::{FileStore, PersistFileError};
use crate::application::buckets::model::{FileIO, FileId};

pub struct Store {
    pub path: PathBuf,
}

impl From<tokio::io::Error> for PersistFileError {
    fn from(e: tokio::io::Error) -> Self {
        PersistFileError::Unknown(Box::new(e))
    }
}

impl FileStore for Store {
    async fn persist<FileIOType: FileIO>(
        &self,
        file_data: &mut FileIOType,
        file_id: &FileId,
    ) -> Result<(), PersistFileError> {
        let file_path = self.path.join(&file_id.0);
        let mut file = File::create(file_path).await?;

        let mut buf = [0; 4096];
        let mut bytes_read = file_data.read(&mut buf).await?;
        // TODO bytes_read == 0 does not strictly mean that the data has
        // reached EOF; consider implementing a more robust solution.
        while bytes_read > 0 {
            let chunk = &buf[0..bytes_read];
            file.write_all(chunk).await?;

            bytes_read = file_data.read(&mut buf).await?;
        }

        file.flush().await?;

        Ok(())
    }

    // async fn fetch<T: FileData>(&self, file_id: &str) -> Result<T, FetchFileError> {
    //     todo!()
    // }
}

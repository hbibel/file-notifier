#![allow(dead_code)]
// TODO

use std::io;
use std::path::{Path, PathBuf};

use tempfile::tempdir;

use crate::application::buckets::file_store::{FileStore, PersistFileError};
use crate::application::buckets::model::{FileData, FileId, Timestamp};

pub struct Store {
    dir: Box<PathBuf>,
}

impl FileStore for Store {
    async fn persist<T: FileData>(
        &self,
        file_data: &T,
        file_id: &FileId,
        timestamp: &Timestamp,
    ) -> Result<(), PersistFileError> {
        todo!()
    }

    async fn fetch<T: FileData>(
        &self,
        file_id: &str,
        timestamp: &Timestamp,
    ) -> Result<T, PersistFileError> {
        todo!()
    }
}

impl Store {
    pub fn new() -> io::Result<Self> {
        let dir = tempdir()?.path();

        Ok(Store {
            dir: Box::new(dir.to_owned()),
        })
    }
}

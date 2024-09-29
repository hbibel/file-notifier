use chrono::{DateTime, Utc};
use tokio::io::{AsyncRead, AsyncWrite};

use super::super::clients::model::ClientId;

#[derive(Clone)]
pub struct FileMeta {
    pub id: FileId,
    pub name: String,
}

pub trait FileIO: AsyncRead + AsyncWrite + Unpin {}

pub struct File<DataT: FileIO> {
    pub meta_data: FileMeta,
    pub data: DataT,
}

pub struct Bucket {
    pub id: BucketId,
    pub name: String,
    pub owner_id: ClientId,
}

#[derive(Clone, PartialEq, Eq)]
pub struct BucketId(pub u32);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FileId(pub String);

impl std::fmt::Display for FileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FileId {
    pub fn new(s: String) -> FileId {
        FileId(s)
    }
}

pub type Timestamp = DateTime<Utc>;

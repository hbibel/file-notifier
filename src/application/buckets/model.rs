use chrono::{DateTime, Utc};

use super::super::clients::model::ClientId;

#[derive(Clone)]
pub struct FileMeta {
    pub id: FileId,
    pub name: String,
}

pub trait FileData: Iterator<Item = u8> {}
pub struct File<I: FileData> {
    pub meta_data: FileMeta,
    pub data: I,
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

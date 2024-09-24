use chrono::{DateTime, Utc};

use super::super::clients::model::ClientId;

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
    owner_id: ClientId,
}

pub struct BucketId(u32);

pub struct FileId(String);

impl FileId {
    pub fn new(s: String) -> FileId {
        FileId(s)
    }
}

pub type Timestamp = DateTime<Utc>;

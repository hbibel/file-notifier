#![allow(dead_code)]
// TODO

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientId(u32);

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Client {
    id: ClientId,
    name: String,
}

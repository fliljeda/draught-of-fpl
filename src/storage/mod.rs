use crate::client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::atomic::AtomicI32;

// Module to fetch and update the storage.

#[derive(Deserialize, Serialize, Debug)]
pub struct Table {
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub name: String,
    pub points: AtomicI32,
}

#[allow(dead_code)]
pub fn from_str(data: &str) -> Result<Table, serde_json::Error> {
    serde_json::from_str(data)
}

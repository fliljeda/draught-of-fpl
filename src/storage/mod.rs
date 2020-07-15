
use crate::structs;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Module to fetch and update the storage.

#[derive(Debug)]
pub struct Table {
    pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub points: i32,
}

#[derive(Debug)]
pub struct FplEndpoints {
    pub details: Option<structs::Details>,
    pub game: Option<structs::Game>,
    pub live: Option<structs::Live>,
    pub static_info: Option<structs::StaticInfo>,
    pub teams_gw: HashMap<u32, Option<structs::TeamGw>>,
    pub teams_info: HashMap<u32, Option<structs::TeamInfo>>,
}

#[derive(Debug)]
pub struct CalculatedValues {
    pub table: Arc<RwLock<Table>>,
}

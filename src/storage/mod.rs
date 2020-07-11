
use crate::structs;
use std::collections::HashMap;

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
    pub details: structs::Details,
    pub game: structs::Game,
    pub live: structs::Live,
    pub static_info: structs::StaticInfo,
    pub teams_gw: HashMap<u32, structs::teamgw::TeamGw>,
    pub teams_info: HashMap<u32, structs::teaminfo::TeamInfo>,
}


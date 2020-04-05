use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamGw {
    pub picks: Vec<Pick>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pick {
    pub element: i32,
    pub position: i32,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub multiplier: i32,
}

#[allow(dead_code)]
pub fn from_str(data: &str) -> Result<TeamGw, serde_json::Error> {
    serde_json::from_str(data)
}

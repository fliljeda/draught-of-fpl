use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamGw {
    pub picks: Vec<Pick>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pick {
    pub element: i32,
    pub position: i32,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub multiplier: i32,
}


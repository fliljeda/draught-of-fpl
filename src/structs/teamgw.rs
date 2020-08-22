use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamGw {
    pub picks: Vec<Pick>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pick {
    pub element: Option<i32>,
    pub position: Option<i32>,
    pub is_captain: Option<bool>,
    pub is_vice_captain: Option<bool>,
    pub multiplier: Option<i32>,
}


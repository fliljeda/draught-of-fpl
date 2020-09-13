use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamGw {
    pub picks: Vec<Pick>,
    pub subs: Vec<Substitution>,
    //pub entry_history: _, //empty {} in all available endpoints
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pick {
    pub element: i32,
    pub position: i32,
    pub is_captain: Option<bool>,
    pub is_vice_captain: Option<bool>,
    pub multiplier: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Substitution {
    pub element_in: i32,
    pub element_out: i32,
    pub event: i32,
}

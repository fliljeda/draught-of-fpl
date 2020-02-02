use serde::{
    Deserialize,
    Serialize,
};
use serde_json;

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamInfo {
    pub entry: Entry,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub event_points: i32,
    //pub favourite_team	null
    pub id: i32,
    pub league_set: Vec<i32>,
    pub name: String,
    pub overall_points: i32,
    pub player_first_name: String,
    pub player_last_name: String,
    pub region_name: String,
    pub region_code_short: String,
    pub region_code_long: String,
    pub started_event: i32,
    pub transactions_event: i32,
    pub transactions_total: i32,
}

#[allow(dead_code)]
pub fn from_str(data: &str) -> Result<TeamInfo, serde_json::Error> {
    serde_json::from_str(data)
}
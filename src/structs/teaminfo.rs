use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamInfo {
    pub entry: Entry,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub event_points: i32,
    pub favourite_team:	Option<i32>,
    pub id: Option<i32>,
    pub league_set: Vec<i32>,
    pub name: String,
    pub overall_points: i32,
    pub player_first_name: String,
    pub player_last_name: String,
    pub region_name: Option<String>,
    pub region_code_short: Option<String>,
    pub region_code_long: Option<String>,
    pub started_event: Option<i32>,
    pub transactions_event: Option<i32>,
    pub transactions_total: Option<i32>,
}


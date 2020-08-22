use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamInfo {
    pub entry: Entry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub event_points: Option<i32>,
    pub favourite_team:	Option<i32>,
    pub id: Option<i32>,
    pub league_set: Vec<i32>,
    pub name: Option<String>,
    pub overall_points: Option<i32>,
    pub player_first_name: Option<String>,
    pub player_last_name: Option<String>,
    pub region_name: Option<String>,
    pub region_code_short: Option<String>,
    pub region_code_long: Option<String>,
    pub started_event: Option<i32>,
    pub transactions_event: Option<i32>,
    pub transactions_total: Option<i32>,
}


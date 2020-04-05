use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Debug)]
pub struct Details {
    pub standings: Vec<TeamStandings>,
    pub league: LeagueInfo,
    pub league_entries: Vec<LeagueEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamStandings {
    pub total: u32,
    pub event_total: u32,
    pub rank: u32,
    pub last_rank: u32,
    pub league_entry: u64,
    pub rank_sort: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeagueInfo {
    pub closed: bool,
    pub make_code_public: bool,
    pub trades: String,
    pub scoring: String,
    pub variety: String,
    pub start_event: u32,
    pub draft_pick_time_limit: u32,
    pub max_entries: u32,
    pub ko_rounds: u32,
    pub draft_dt: String,
    pub draft_status: String,
    pub admin_entry: u32,
    pub name: String,
    pub min_entries: u32,
    pub stop_event: u32,
    pub draft_tz_show: String,
    pub transaction_mode: String,
    pub id: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeagueEntry {
    pub player_first_name: String,
    pub joined_time: String,
    pub short_name: String,
    pub entry_id: u32,
    pub id: u32,
    pub waiver_pick: u32,
    pub player_last_name: String,
    pub entry_name: String,
}

#[allow(dead_code)]
pub fn from_str(data: &str) -> Result<Details, serde_json::Error> {
    serde_json::from_str(data)
}


use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    pub standings: Vec<TeamStandings>,
    pub league: LeagueInfo,
    pub league_entries: Vec<LeagueEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamStandings {
    pub total: Option<u32>,
    pub event_total: Option<u32>,
    pub rank: Option<u32>,
    pub last_rank: Option<u32>,
    pub league_entry: Option<u64>,
    pub rank_sort: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueInfo {
    pub closed: Option<bool>,
    pub make_code_public: Option<bool>,
    pub trades: Option<String>,
    pub scoring: Option<String>,
    pub variety: Option<String>,
    pub start_event: Option<u32>,
    pub draft_pick_time_limit: Option<u32>,
    pub max_entries: Option<u32>,
    pub ko_rounds: Option<u32>,
    pub draft_dt: Option<String>,
    pub draft_status: Option<String>,
    pub admin_entry: Option<u32>,
    pub name: Option<String>,
    pub min_entries: Option<u32>,
    pub stop_event: Option<u32>,
    pub draft_tz_show: Option<String>,
    pub transaction_mode: Option<String>,
    pub id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueEntry {
    pub player_first_name: Option<String>,
    pub joined_time: Option<String>,
    pub short_name: Option<String>,
    pub entry_id: Option<u32>,
    pub id: Option<u32>,
    pub waiver_pick: Option<u32>,
    pub player_last_name: Option<String>,
    pub entry_name: Option<String>,
}


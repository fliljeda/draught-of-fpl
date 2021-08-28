
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Details {
    pub standings: Vec<TeamStandings>,
    pub league: LeagueInfo,
    pub matches: Option<Vec<H2HMatch>>,
    pub league_entries: Vec<LeagueEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamStandings {
    pub total: i32,
    pub event_total: Option<u32>, // Classic scoring only
    pub rank: Option<u32>,
    pub last_rank: Option<u32>,
    pub league_entry: u32,
    pub rank_sort: Option<u32>,
    pub matches_drawn: Option<u32>, // H2H scoring only
    pub matches_lost: Option<u32>, // H2H scoring only
    //pub matches_played: Option<u32>, // H2H scoring only. Seems bugged as of 2021-08-21: always set to 38 (#gameweeks)
    pub matches_won: Option<u32>, // H2H scoring only
    pub points_against: Option<i32>, // H2H scoring only
    pub points_for: Option<i32>, // H2H scoring only
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeagueInfo {
    pub closed: Option<bool>,
    pub make_code_public: Option<bool>,
    pub trades: Option<String>,
    pub scoring: String, // "h" for h2h and "c" for classic scoring
    pub variety: Option<String>,
    pub start_event: Option<u32>,
    pub draft_pick_time_limit: Option<u32>,
    pub max_entries: Option<u32>,
    pub ko_rounds: Option<u32>,
    pub draft_dt: Option<String>,
    pub draft_status: Option<String>,
    pub admin_entry: Option<u32>,
    pub name: String,
    pub min_entries: Option<u32>,
    pub stop_event: Option<u32>,
    pub draft_tz_show: Option<String>,
    pub transaction_mode: Option<String>,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeagueEntry {
    pub player_first_name: Option<String>,
    pub joined_time: Option<String>,
    pub short_name: Option<String>,
    pub entry_id: u32,
    pub id: u32,
    pub waiver_pick: Option<u32>,
    pub player_last_name: Option<String>,
    pub entry_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct H2HMatch {
    pub event: u32,
    pub league_entry_1: u32,
    pub league_entry_1_points: u32,
    pub league_entry_2: u32,
    pub league_entry_2_points: u32,
    pub started: bool,
    pub finished: bool,

    // Both Null as of 2021-08-21 for a finished H2HMatch
    //pub winning_league_entry: Option<u32>,
    //pub winning_method: Option<String>, // Unclear what this is as this is developed before first GW is over, but probably something like "w"/"d" and if "d" the winning entry is useless

}

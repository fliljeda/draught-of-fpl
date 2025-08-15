use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaticInfo {
    pub elements: Vec<Element>,
    pub element_types: Vec<ElementType>,
    pub element_stats: Vec<ElementStats>,
    pub events: Events,
    pub fixtures: HashMap<i32, Vec<Fixture>>, //Contains next three fixtures (current gw + 1/2/3) as keys
    pub settings: Settings,
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element {
    pub web_name: String,
    pub goals_conceded: Option<i32>,
    pub in_dreamteam: Option<bool>,
    pub news_return: Option<i32>,
    pub team: u32,
    pub influence: Option<String>,
    pub bonus: Option<i32>,
    pub news: Option<String>,
    pub clean_sheets: Option<i32>,
    pub bps: Option<i32>,
    pub creativity: Option<String>,
    pub ep_this: Option<i32>,
    pub status: String,
    pub penalties_missed: Option<i32>,
    pub id: Option<i32>,
    pub element_type: i32,
    pub news_updated: Option<i32>,
    pub chance_of_playing_this_round: Option<i32>,
    pub dreamteam_count: Option<i32>,
    pub first_name: String,
    pub red_cards: Option<i32>,
    pub chance_of_playing_next_round: Option<i32>,
    pub ict_index: Option<String>,
    pub code: Option<i32>,
    pub added: Option<String>,
    pub second_name: String,
    pub squad_number: Option<i32>,
    pub draft_rank: Option<i32>,
    pub total_points: Option<i32>,
    pub saves: Option<i32>,
    pub assists: Option<i32>,
    pub own_goals: Option<i32>,
    pub penalties_saved: Option<i32>,
    pub ep_next: Option<String>,
    pub points_per_game: Option<String>,
    pub news_added: Option<String>,
    pub threat: Option<String>,
    pub form: Option<String>,
    pub goals_scored: Option<i32>,
    pub yellow_cards: Option<i32>,
    pub minutes: Option<i32>,
    pub event_points: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElementType {
    pub id: Option<i32>,
    pub singular_name: Option<String>,
    pub singular_name_short: Option<String>,
    pub plural_name: Option<String>,
    pub plural_name_short: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElementStats {
    pub name: Option<String>,
    pub label: Option<String>,
    pub abbreviation: Option<String>,
    pub is_match_stat: Option<bool>,
    pub match_stat_order: Option<i32>,
    pub sort: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Events {
    pub current: Option<i32>,
    pub data: Vec<EventData>,
    pub next: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventData {
    pub average_entry_score: Option<i32>,
    pub deadline_time: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub finished: Option<bool>,
    pub highest_scoring_entry: Option<i32>,
    pub waivers_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fixture {
    pub id: Option<i32>,
    pub started: Option<bool>,
    pub code: Option<i32>,
    pub finished: Option<bool>,
    pub finished_provisional: Option<bool>,
    pub kickoff_time: Option<String>,
    pub minutes: Option<i32>,
    pub provisional_start_time: Option<bool>,
    pub team_a_score: Option<i32>,
    pub team_h_score: Option<i32>,
    pub event: Option<i32>,
    pub team_a: Option<i32>,
    pub team_h: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub league: League,
    pub scoring: Scoring,
    pub squad: Squad,
    pub transactions: Transactions,
    pub ui: Ui,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct League {
    pub default_entries: Option<i32>,
    pub draft_reminder_hours: Vec<i32>,
    pub draft_postpone_hours: Option<i32>,
    pub draft_pushback_times: Option<i32>,
    pub h2h_draw: Option<i32>,
    pub h2h_lose: Option<i32>,
    pub h2h_win: Option<i32>,
    pub max_entries: Option<i32>,
    pub min_entries: Option<i32>,
    pub private_max: Option<i32>,
    pub public_draft_delay_minutes: Option<i32>,
    pub public_draft_tz_default: Option<String>,
    pub public_entry_sizes: Vec<i32>,
    pub public_max: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scoring {
    pub long_play_limit: Option<i32>,
    pub short_play: Option<i32>,
    pub long_play: Option<i32>,
    pub concede_limit: Option<i32>,
    pub goals_conceded_GKP: Option<i32>,
    pub goals_conceded_DEF: Option<i32>,
    pub goals_conceded_MID: Option<i32>,
    pub goals_conceded_FWD: Option<i32>,
    pub saves_limit: Option<i32>,
    pub saves: Option<i32>,
    pub goals_scored_GKP: Option<i32>,
    pub goals_scored_DEF: Option<i32>,
    pub goals_scored_MID: Option<i32>,
    pub goals_scored_FWD: Option<i32>,
    pub assists: Option<i32>,
    pub clean_sheets_GKP: Option<i32>,
    pub clean_sheets_DEF: Option<i32>,
    pub clean_sheets_MID: Option<i32>,
    pub clean_sheets_FWD: Option<i32>,
    pub penalties_saved: Option<i32>,
    pub penalties_missed: Option<i32>,
    pub yellow_cards: Option<i32>,
    pub red_cards: Option<i32>,
    pub own_goals: Option<i32>,
    pub bonus: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Squad {
    pub size: Option<i32>,
    pub select_GKP: Option<i32>,
    pub select_DEF: Option<i32>,
    pub select_MID: Option<i32>,
    pub select_FWD: Option<i32>,
    pub play: Option<i32>,
    pub min_play_GKP: Option<i32>,
    pub max_play_GKP: Option<i32>,
    pub min_play_DEF: Option<i32>,
    pub max_play_DEF: Option<i32>,
    pub min_play_MID: Option<i32>,
    pub max_play_MID: Option<i32>,
    pub min_play_FWD: Option<i32>,
    pub max_play_FWD: Option<i32>,
    pub position_type_locks: HashMap<i32, String>,
    pub captains_disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transactions {
    pub new_element_locked_hours: Option<i32>,
    pub trade_veto_minimum: Option<i32>,
    pub trade_veto_hours: Option<i32>,
    pub waivers_before_start_min_hours: Option<i32>,
    pub waivers_before_deadline_hours: Option<i32>,
    pub waivers_before_deadline_hours_event: HashMap<i32, i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ui {
    //special_shirt_exclusions: 	[]
    pub use_special_shirts: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub code: i32,
    pub id: u32,
    pub name: String,
    pub short_name: String,
}

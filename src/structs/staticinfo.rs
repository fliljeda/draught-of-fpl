use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StaticInfo {
    pub elements: Vec<Element>,
    pub element_types: Vec<ElementType>,
    pub element_stats: Vec<ElementStats>,
    pub events: Events,
    pub fixtures: HashMap<i32, Vec<Fixture>>, //Contains next three fixtures (current gw + 1/2/3) as keys
    pub settings: Settings,
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    pub web_name: String,
    pub goals_conceded: i32,
    pub in_dreamteam: bool,
    //pub news_return: null,
    pub team: i32,
    pub influence: String,
    pub bonus: i32,
    pub news: String,
    pub clean_sheets: i32,
    pub bps: i32,
    pub creativity: String,
    //pub ep_this: null,
    pub status: String,
    pub penalties_missed: i32,
    pub id: i32,
    pub element_type: i32,
    //news_updated: null,
    //chance_of_playing_this_round: null,
    pub dreamteam_count: i32,
    pub first_name: String,
    pub red_cards: i32,
    //chance_of_playing_next_round: null,
    pub ict_index: String,
    pub code: i32,
    pub added: String,
    pub second_name: String,
    //squad_number: null,
    pub draft_rank: i32,
    pub total_points: i32,
    pub saves: i32,
    pub assists: i32,
    pub own_goals: i32,
    pub penalties_saved: i32,
    //pub ep_next: null,
    pub points_per_game: String,
    //news_added: null,
    pub threat: String,
    pub form: String,
    pub goals_scored: i32,
    pub yellow_cards: i32,
    pub minutes: i32,
    pub event_points: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ElementType {
    pub id: i32,
    pub singular_name: String,
    pub singular_name_short: String,
    pub plural_name: String,
    pub plural_name_short: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ElementStats {
    pub name: String,
    pub label: String,
    pub abbreviation: String,
    pub is_match_stat: bool,
    //pub match_stat_order: null
    pub sort: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Events {
    pub current: i32,
    pub data: Vec<EventData>,
    pub next: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventData {
    //pub average_entry_score: null
    pub deadline_time: String,
    pub id: i32,
    pub name: String,
    pub finished: bool,
    //pub highest_scoring_entry:	null
    pub waivers_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fixture {
    pub id: i32,
    pub started: bool,
    pub code: i32,
    pub finished: bool,
    pub finished_provisional: bool,
    pub kickoff_time: String,
    pub minutes: i32,
    pub provisional_start_time: bool,
    //pub team_a_score:	null
    //pub team_h_score:	null
    pub event: i32,
    pub team_a: i32,
    pub team_h: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub league: League,
    pub scoring: Scoring,
    pub squad: Squad,
    pub transactions: Transactions,
    pub ui: Ui,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct League {
    pub default_entries: i32,
    pub draft_reminder_hours: Vec<i32>,
    pub draft_postpone_hours: i32,
    pub draft_pushback_times: i32,
    pub h2h_draw: i32,
    pub h2h_lose: i32,
    pub h2h_win: i32,
    pub max_entries: i32,
    pub min_entries: i32,
    pub private_max: i32,
    pub public_draft_delay_minutes: i32,
    pub public_draft_tz_default: String,
    pub public_entry_sizes: Vec<i32>,
    pub public_max: i32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Scoring {
    pub long_play_limit: i32,
    pub short_play: i32,
    pub long_play: i32,
    pub concede_limit: i32,
    pub goals_conceded_GKP: i32,
    pub goals_conceded_DEF: i32,
    pub goals_conceded_MID: i32,
    pub goals_conceded_FWD: i32,
    pub saves_limit: i32,
    pub saves: i32,
    pub goals_scored_GKP: i32,
    pub goals_scored_DEF: i32,
    pub goals_scored_MID: i32,
    pub goals_scored_FWD: i32,
    pub assists: i32,
    pub clean_sheets_GKP: i32,
    pub clean_sheets_DEF: i32,
    pub clean_sheets_MID: i32,
    pub clean_sheets_FWD: i32,
    pub penalties_saved: i32,
    pub penalties_missed: i32,
    pub yellow_cards: i32,
    pub red_cards: i32,
    pub own_goals: i32,
    pub bonus: i32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Squad {
    pub size: i32,
    pub select_GKP: i32,
    pub select_DEF: i32,
    pub select_MID: i32,
    pub select_FWD: i32,
    pub play: i32,
    pub min_play_GKP: i32,
    pub max_play_GKP: i32,
    pub min_play_DEF: i32,
    pub max_play_DEF: i32,
    pub min_play_MID: i32,
    pub max_play_MID: i32,
    pub min_play_FWD: i32,
    pub max_play_FWD: i32,
    pub position_type_locks: HashMap<i32, String>,
    pub captains_disabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions {
    pub new_element_locked_hours: i32,
    pub trade_veto_minimum: i32,
    pub trade_veto_hours: i32,
    pub waivers_before_start_min_hours: i32,
    pub waivers_before_deadline_hours: i32,
    pub waivers_before_deadline_hours_event: HashMap<i32, i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ui {
    //special_shirt_exclusions: 	[]
    pub use_special_shirts: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub code: i32,
    pub id: i32,
    pub name: String,
    pub short_name: String,
}

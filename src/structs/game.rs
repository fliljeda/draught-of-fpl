use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub current_event: i32,
    pub current_event_finished: bool,
    pub next_event: i32,
    pub processing_status: String,
    pub trades_time_for_approval: bool,
    pub waivers_processed: bool,
}

#[allow(dead_code)]
pub fn from_str(data: &str) -> Result<Game, serde_json::Error> {
    serde_json::from_str(data)
}

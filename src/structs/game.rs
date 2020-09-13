use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub current_event: Option<u32>,
    pub current_event_finished: Option<bool>,
    pub next_event: Option<u32>,
    pub processing_status: Option<String>,
    pub trades_time_for_approval: Option<bool>,
    pub waivers_processed: Option<bool>,
}

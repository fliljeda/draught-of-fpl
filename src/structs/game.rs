use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub current_event: i32,
    pub current_event_finished: bool,
    pub next_event: i32,
    pub processing_status: String,
    pub trades_time_for_approval: bool,
    pub waivers_processed: bool,
}

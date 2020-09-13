use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueTable {
    pub entries: Vec<Entry>,
    pub code: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub team_name: String,
    pub points: i32,
    pub projected_points: i32,
    pub owner_name: String,
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: u32,
    pub full_name: String,
    pub display_name: String,
    pub team: Team,
    pub team_pos: Position,
    pub points: i32,
    pub bps: i32,
    pub projected_points: i32,
    pub point_sources: Vec<PointSource>,
    pub on_field: bool,
    pub has_played: bool,
    pub fixtures_finished: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub code: u32,
    pub shirt_url: String,
    pub gk_shirt_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub number: u32,
    pub name: String,
}

impl Position {
    pub fn from_number(number: u32) -> Position {
        let name = match number {
            1 => "GK",
            2 => "DEF",
            3 => "MID",
            4 => "FWD",
            _ => ""
        };
        let name = String::from(name);
        Position {
            number,
            name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PointSource {
    pub name: String,
    pub points_total: i32,
    pub value: i32,
    pub fixture: u32,
    pub stat: String,

}

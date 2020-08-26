use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct LeagueTable {
    entries: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    team_name: String,
    points: i32,
    projected_points: i32,
    owner_name: String,
    players: Vec<Player>,
}

#[derive(Deserialize, Debug)]
pub struct Player {
    id: u32,
    full_name: String,
    display_name: String,
    team_pos: i32,
    points: i32,
    bonus_points: i32,
    projected_points: i32,
    point_sources: Vec<PointSource>,
}

#[derive(Deserialize, Debug)]
pub struct PointSource {
    name: String,
    points_total: i32,
    points_per: i32,
    times: u32,
}

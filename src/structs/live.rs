use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Live {
    pub elements: HashMap<String, Element>,
    pub fixtures: Vec<Fixture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    //IMPORTAN TO NOTE ABOUT EXPLAIN IS THAT THE INSIDE VEC IS JUST DUMMY ELEMENT TO HOLD
    //EITHER A VECTOR OF POINTSOURCES AND WHAT TEAM THEY FACED WHEN ON INDEX 0 AND 1 RESPECTIVELY
    //SO: explain[x][0] is array of point sources, explain[x][1] is the team they faced
    pub explain: Vec<Vec<ElementFixture>>,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ElementFixture {
    Points(Vec<Point>),
    OpposingTeam(i32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    name: String,
    points: i32,
    value: i32,
    stat: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub minutes: i32,
    pub goals_scored: i32,
    pub assists: i32,
    pub clean_sheets: i32,
    pub goals_conceded: i32,
    pub own_goals: i32,
    pub penalties_saved: i32,
    pub penalties_missed: i32,
    pub yellow_cards: i32,
    pub red_cards: i32,
    pub saves: i32,
    pub bonus: i32,
    pub bps: i32,
    pub influence: f64,
    pub creativity: f64,
    pub threat: f64,
    pub ict_index: f64,
    pub total_points: i32,
    pub in_dreamteam: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fixture {
    pub id: i32,
    pub started: bool,
    pub stats: Vec<FixtureStats>,
    pub code: i32,
    pub finished: bool,
    pub finished_provisional: bool,
    pub kickoff_time: String,
    pub minutes: i32,
    pub provisional_start_time: bool,
    pub team_a_score: i32,
    pub team_h_score: i32,
    pub event: i32,
    pub team_a: i32,
    pub team_h: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixtureStats {
    pub s: String,           //type of stat (eg. red_cards, saves, bps, bonus)
    pub h: Vec<FixtureStat>, //home
    pub a: Vec<FixtureStat>, //away
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixtureStat {
    element: i32,
    value: i32,
}

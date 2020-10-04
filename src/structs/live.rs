use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Live {
    pub elements: HashMap<String, Element>,
    pub fixtures: Vec<Fixture>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element {
    //IMPORTAN TO NOTE ABOUT EXPLAIN IS THAT THE INSIDE VEC IS JUST DUMMY ELEMENT TO HOLD
    //EITHER A VECTOR OF POINTSOURCES AND WHAT FIXTURE WHEN ON INDEX 0 AND 1 RESPECTIVELY
    //SO: explain[x][0] is array of point sources, explain[x][1] is the fixture id
    pub explain: Vec<Vec<PointsOrFixture>>,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PointsOrFixture {
    Points(Vec<Point>),
    Fixture(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
    pub name: String,
    pub points: i32,
    pub value: i32,
    pub stat: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub minutes: i32,
    pub goals_scored: Option<i32>,
    pub assists: Option<i32>,
    pub clean_sheets: Option<i32>,
    pub goals_conceded: Option<i32>,
    pub own_goals: Option<i32>,
    pub penalties_saved: Option<i32>,
    pub penalties_missed: Option<i32>,
    pub yellow_cards: Option<i32>,
    pub red_cards: Option<i32>,
    pub saves: Option<i32>,
    pub bonus: Option<i32>,
    pub bps: i32,
    pub influence: Option<f64>,
    pub creativity: Option<f64>,
    pub threat: Option<f64>,
    pub ict_index: Option<f64>,
    pub total_points: i32,
    pub in_dreamteam: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fixture {
    pub id: u32,
    pub started: bool,
    pub stats: Vec<FixtureStats>,
    pub code: Option<i32>,
    pub finished: bool,
    pub finished_provisional: bool,
    pub kickoff_time: Option<String>,
    pub minutes: i32,
    pub provisional_start_time: Option<bool>,
    pub team_a_score: Option<i32>,
    pub team_h_score: Option<i32>,
    pub event: Option<i32>,
    pub team_a: Option<i32>,
    pub team_h: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixtureStats {
    pub s: String,           //type of stat (eg. red_cards, saves, bps, bonus)
    pub h: Vec<FixtureStat>, //home
    pub a: Vec<FixtureStat>, //away
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixtureStat {
    pub element: u32,
    pub value: i32,
}

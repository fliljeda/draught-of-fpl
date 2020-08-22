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
    name: Option<String>,
    points: Option<i32>,
    value: Option<i32>,
    stat: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub minutes: Option<i32>,
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
    pub bps: Option<i32>,
    pub influence: Option<f64>,
    pub creativity: Option<f64>,
    pub threat: Option<f64>,
    pub ict_index: Option<f64>,
    pub total_points: Option<i32>,
    pub in_dreamteam: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fixture {
    pub id: Option<i32>,
    pub started: Option<bool>,
    pub stats: Vec<FixtureStats>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct FixtureStats {
    pub s: Option<String>,           //type of stat (eg. red_cards, saves, bps, bonus)
    pub h: Vec<FixtureStat>, //home
    pub a: Vec<FixtureStat>, //away
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixtureStat {
    element: Option<i32>,
    value: Option<i32>,
}

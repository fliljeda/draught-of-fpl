use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueTable {
    // An array of the entries of the draft league
    pub entries: Vec<Entry>,

    // The code of the draft league
    pub code: u32,

    // The name of the draft league
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    // The name of the draft league team
    pub team_name: String,

    // The total number of points of this team all season
    pub total_points: i32,

    // The total number of projected points of this team all season
    pub total_projected_points: i32,

    // The current number of total points of the team, as calculated by the official FPL site
    pub gw_points: i32,

    // The current number of points of the team, by calculating current metrics such as bps and
    // certain substitutions
    pub gw_projected_points: i32,

    // List of objects containing simple information about the projected points
    pub projected_points_explanations: Vec<ProjectedPointsExplanation>,

    // Name of the owner of the FPL team
    pub owner_name: String,

    // Array containing detailed information of each player
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectedPointsExplanation {
    // Name of player
    pub name: String,

    // Bonus points of the player if exists (diff in points and projected points)
    pub bonus_points: Option<i32>,

    // Points of the players if subbed on (not projected)
    pub subbed_points: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    // The ID of the player
    pub id: u32,

    // The full name of the player, concatenation of first name and second name
    pub full_name: String,

    // The display name used by FPL
    pub display_name: String,

    // The team the player plays for
    pub team: Team,

    // The position the player has on FPL
    pub team_pos: Position,

    // The number of points of the player on FPL
    pub points: i32,

    // The number of points in the BPS (bonus point system)
    pub bps: i32,

    // The current projected number of points for the player considering information such as bps and
    // substitutions
    pub projected_points: i32,

    // An array containing the sources of the points of the player (not including projected points)
    pub point_sources: Vec<PointSource>,

    // Whether or not the player is on the field either selected by the team owner or if
    // substituted in
    pub on_field: bool,

    // The pick number of the player on the FPL teams. Where 12-15 are the bench and the rest are
    // chosen by the team owner as playing
    pub pick_number: i32,

    // Whether or not this player has played any minutes this game week, therefore qualifying for
    // points
    pub has_played: bool,

    // Whether or not this player's fixtures are finished
    pub fixtures_finished: bool,

    // Whether or not the player has any fixtures this gameweek
    pub has_upcoming_fixtures: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    // The team ID. Identifies the team in the FPL Json structures
    pub id: u32,

    // The name of the team
    pub name: String,

    // The short name of the team
    pub short_name: String,

    // The code of the team. Identifies the team in at least the icon URLs
    pub code: u32,

    // The URL pointing to a small image of this team's outfield shirt
    pub shirt_url: String,

    // The URL pointing to a small image of this team's goalkeeper shirt
    pub gk_shirt_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    // The number of this position as used by FPL (1 GK, 2 DEF, 3 MID, 4 FWD)
    pub number: u32,

    // The short name of the position (GK, DEF, MID, FWD)
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
    // The name of this point source (eg. Goals scored/Assists/Clean sheets)
    pub name: String,

    // The total number of points of this source as determined by FPL,
    // will vary by position and amount
    pub points_total: i32,

    // The amount of times this point source has happened
    pub amount: i32,

    // The fixture ID that this point comes from
    pub fixture: u32,

    // The identifying string for this point source type (used by FPL to calculate points
    pub stat: String,
}

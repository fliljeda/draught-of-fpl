// Computes the basic properties from endpoint objects
use crate::storage::{
    table::{
        PointSource as TablePointSource,
        Team as TableTeam,
    },
    FplEndpoints
};
use crate::structs::{
    live::{
        Fixture as LiveFixture,
        FixtureStat,
        Point as LivePoint,
        PointsOrFixture,
        Element as LiveElement,
        PointsOrFixture::{Fixture, Points},
    },
    teamgw::{
        Pick as TeamGwPick,
        TeamGw,
    },
    staticinfo::{
        Element as StaticElement,
    },
    teaminfo::{
        Entry as TeamInfoEntry
    }
};

// Gets the display name for a FPL player
pub fn get_player_display_name(endpoints: &FplEndpoints, player_id: u32) -> String {
    String::from(&get_player_from_static(endpoints, player_id).web_name)
}

pub fn get_player_full_name(endpoints: &FplEndpoints, player_id: u32) -> String {
    let p = get_player_from_static(endpoints, player_id);
    format!("{} {}", p.first_name, p.second_name)
}

pub fn get_player_team_id(endpoints: &FplEndpoints, player_id: u32) -> u32 {
    get_player_from_static(endpoints, player_id).team
}

pub fn get_player_position(endpoints: &FplEndpoints, player_id: u32) -> u32 {
    get_player_from_static(endpoints, player_id).element_type as u32
}

pub fn get_player_points(endpoints: &FplEndpoints, player_id: u32) -> i32 {
    get_player_from_live(endpoints, player_id).stats.total_points
}

pub fn get_player_bps(endpoints: &FplEndpoints, player_id: u32) -> i32 {
    get_player_from_live(endpoints, player_id).stats.bps
}

// "is on field" happens if the player was selected to play on field or is a part of the substitutes
pub fn compute_player_is_on_field(pick: &TeamGwPick, team_gw: &TeamGw) -> bool {
    let selected_on_field = pick.position <= 11;
    let substituted_in = team_gw.subs.iter().find(|x| x.element_in == pick.element).is_some();
    let substituted_out = team_gw.subs.iter().find(|x| x.element_out == pick.element).is_some();

    (selected_on_field && !substituted_out) || substituted_in
}

pub fn compute_player_has_played(endpoints: &FplEndpoints, player_id: u32) -> bool {
    let player = get_player_from_live(endpoints, player_id);
    player.stats.minutes > 0
}

pub fn compute_player_fixtures_has_finished(endpoints: &FplEndpoints, player_id: u32) -> bool {
    let fixtures = get_player_current_fixtures(endpoints, player_id);
    fixtures.iter().all(|fixture| fixture.finished)
}

pub fn compute_player_has_upcoming_fixtures(endpoints: &FplEndpoints, player_id: u32) -> bool {
    let fixtures = get_player_current_fixtures(endpoints, player_id);
    fixtures.iter().any(|f| !f.started)
}

pub fn compute_player_team(endpoints: &FplEndpoints, player_id: u32) -> TableTeam {
    let team_id = get_player_team_id(endpoints, player_id);
    let team = endpoints.static_info.teams.iter().find(|team| team_id == team.id)
        .expect(&format!("Can't find team with ID: {} for player with id {}", team_id, player_id));
    TableTeam{
        id: team.id as u32,
        name: String::from(&team.name),
        short_name: String::from(&team.short_name),
        code: team.code as u32,
        shirt_url: format!("https://draft.premierleague.com/img/shirts/standard/shirt_{}-36.png", team.code),
        gk_shirt_url: format!("https://draft.premierleague.com/img/shirts/standard/shirt_{}_1-36.png", team.code),
    }
}


// Projected points calculates the sum of the points from all point sources and if calculates how
// many bonus points the BPS would give the player. Does not calculate bonus points if bonus has
// already been applied to the player.
// Returns 0 if there are no fixtures for the player
pub fn get_player_projected_points(endpoints: &FplEndpoints, player_id: u32) -> i32 {
    let player_live = get_player_from_live(endpoints, player_id);
    let fixture_ids = get_player_current_fixtures(endpoints, player_id);

    let mut points = 0;
    for fixture in fixture_ids {
        let point_sources = calculate_point_sources(&player_live.explain, fixture.id);
        let mut bonus_accounted_for = false;
        for p in point_sources {
            points += p.points;
            if p.stat.eq("bonus") {
                bonus_accounted_for = true;
            }
        }
        if !bonus_accounted_for {
            points += calculate_bonus_points(fixture, player_id);
        }
    }

    points
}

pub fn get_player_point_sources(endpoints: &FplEndpoints, player_id: u32) -> Vec<TablePointSource> {
    let mut point_sources = Vec::new();
    let player_live = get_player_from_live(endpoints, player_id);
    let fixture_ids = get_player_current_fixtures(endpoints, player_id);

    for fixture in fixture_ids {
        let live_points = calculate_point_sources(&player_live.explain, fixture.id);
        for live_point in live_points {
            let src = TablePointSource {
                name: live_point.name,
                amount: live_point.value,
                points_total: live_point.points,
                stat: live_point.stat,
                fixture: fixture.id,
            };
            point_sources.push(src);
        }
    }

    point_sources
}

// The body of this function may look stupid. But the data structure chosen by the FPL team is stupid
fn calculate_point_sources(fixtures: &Vec<Vec<PointsOrFixture>>, fixture_id: u32) -> Vec<LivePoint> {
    let fixture = fixtures.iter().find(|x| {
        x.iter().any(|y| {
            return if let Fixture(n) = y {
                *n == fixture_id
            } else {
                false
            };
        })
    }).expect(
        &format!("Could not find fixture {} when computing point sources (parsing explain object)", fixture_id)
    );

    let mut point_sources = Vec::new();
    for pof in fixture {
        if let Points(points) = pof {
            point_sources = points.to_vec();
        }
    }
    point_sources
}

// Calculate bonus points
fn calculate_bonus_points(fixture: &LiveFixture, player_id: u32) -> i32 {
    let fixture_minutes = fixture.minutes;
    for stat in fixture.stats.iter() {
        if stat.s.eq("bps") {
            let mut bps: Vec<FixtureStat> = Vec::new();
            bps.extend(stat.a.to_vec());
            bps.extend(stat.h.to_vec());

            if bps.len() < 3 {
                return 0;
            }

            bps.sort_by_key(|x| std::cmp::Reverse(x.value));

            let player_bps = match bps.iter().find(|x| x.element == player_id) {
                Some(stat) => stat.value,
                None => 0,
            };

            if player_bps < 10 && fixture_minutes < 15 {
                return 0;
            }

            let bonus = match player_bps {
                0 => 0,
                x if x == bps[0].value => 3,
                x if x == bps[1].value => 2,
                x if x == bps[2].value => 1,
                _ => 0,
            };

            return bonus;
        }
    }
    0
}

pub fn get_player_current_fixtures(endpoints: &FplEndpoints, player_id: u32) -> Vec<&LiveFixture> {
    let ids = get_player_current_fixture_ids(endpoints, player_id);
    let mut fixtures = Vec::new();
    let live_fixtures = &endpoints.live.fixtures;
    for fixture in live_fixtures.iter() {
        for id in ids.iter() {
            if *id == fixture.id {
                fixtures.push(fixture);
            }
        }
    }
    fixtures
}

pub fn get_player_current_fixture_ids(endpoints: &FplEndpoints, player_id: u32) -> Vec<u32> {
    let player_live = get_player_from_live(endpoints, player_id);
    let mut fixtures: Vec<u32> = Vec::new();
    for x in player_live.explain.iter() {
        if let Fixture(id) = x[1] {
            fixtures.push(id);
        }
    }
    fixtures
}

pub fn get_league_name(endpoints: &FplEndpoints) -> String {
    String::from(&endpoints.details.league.name)
}

pub fn get_league_id(endpoints: &FplEndpoints) -> u32 {
    endpoints.details.league.id
}

pub fn get_team_name(endpoints: &FplEndpoints, team_id: u32) -> String {
    let placeholder = String::from("<Team Name Unknown>");
    match get_team_info_entry(endpoints, team_id) {
        None => {
            log::warn!("Calculating team name: Did not find team with id {} in endpoints", team_id);
            placeholder
        }
        Some(entry) => {
            return String::from(&entry.name);
        }
    }
}

pub fn get_team_owner_name(endpoints: &FplEndpoints, team_id: u32) -> String {
    let placeholder = String::from("<Team Owner Unknown>");
    match get_team_info_entry(endpoints, team_id) {
        None => {
            log::warn!("Calculating team owner name: Did not find team with id {} in endpoints", team_id);
            placeholder
        }
        Some(entry) => {
            return format!("{} {}", &entry.player_first_name, &entry.player_last_name);
        }
    }
}

fn get_player_from_static(endpoints: &FplEndpoints, player_id: u32) -> &StaticElement {
    let i = (player_id - 1) as usize;
    endpoints.static_info.elements.get(i).expect(
        format!("Player Id {} does not exist in bootstrap-static", player_id).as_str()
    )
}

fn get_player_from_live(endpoints: &FplEndpoints, player_id: u32) -> &LiveElement {
    endpoints.live.elements.get(player_id.to_string().as_str()).expect(
        format!("Player Id {} does not exist in live endpoint", player_id).as_str()
    )
}

fn get_team_info_entry(endpoints: &FplEndpoints, team_id: u32) -> Option<&TeamInfoEntry> {
    endpoints.teams_infos.get(&team_id).and_then(|val_res| {
        Some(&val_res.entry)
    })
}

use std::{thread, time};
use std::sync::{Arc, RwLock};

use crate::propcomp;
use crate::storage::{FplEndpoints, LeagueTable};
use crate::storage::table::{Entry as TableEntry, ProjectedPointsExplanation};
use crate::storage::table::Player as TablePlayer;
use crate::storage::table::Position as PlayerPosition;

#[allow(dead_code)]
pub fn league_table_computer(lock: Arc<RwLock<LeagueTable>>, endpoints_lock: Arc<RwLock<FplEndpoints>>) {
    loop {
        {
            let sleep_ms = 30_000;
            log::trace!("Sleeping computer thread for {} ms", sleep_ms);
            thread::sleep(time::Duration::from_millis(sleep_ms));
        }

        let endpoints = match endpoints_lock.read() {
            Ok(e) => (*e).clone(),
            Err(e) => {
                log::error!("Could not grab read lock for endpoints in computer thread, {}", e);
                continue;
            }
        };

        log::info!("Computing new league table");
        let new_table = compute_new_league_table(endpoints);
        match new_table {
            Some(new_table) => {
                match lock.write() {
                    Ok(mut t) => {
                        log::trace!("Grabbed the league table lock");
                        *t = new_table;
                    }
                    Err(e) => {
                        log::error!("Could not grab write lock for table: {}", e);
                    }
                };
            }
            None => (),
        }
    }
}

pub fn compute_new_league_table(endpoints: FplEndpoints) -> Option<LeagueTable> {
    let mut entries = compute_league_entries(&endpoints);
    entries.sort_by_key(|x| std::cmp::Reverse(x.total_points));
    let table = LeagueTable {
        entries,
        code: propcomp::get_league_id(&endpoints),
        name: propcomp::get_league_name(&endpoints),
    };
    Some(table)
}

fn compute_league_entries(endpoints: &FplEndpoints) -> Vec<TableEntry> {
    let entries: Vec<TableEntry> = endpoints.details.league_entries.iter()
        .map(|entry| {
            compute_league_entry(endpoints, entry.entry_id)
        })
        .collect();

    entries
}

fn get_total_points_before_gw(endpoints: &FplEndpoints, id: u32) -> i32 {
    let team_info = &(endpoints.teams_infos.get(&id).unwrap().entry);
    team_info.overall_points - team_info.event_points
}

fn compute_league_entry(endpoints: &FplEndpoints, id: u32) -> TableEntry {
    let team_code = id;
    let players = extract_players(endpoints, id);

    let owner_name = propcomp::get_team_owner_name(endpoints, id);
    let team_name = propcomp::get_team_name(endpoints, id);

    let gw_points = players.iter()
        .filter(|p| p.on_field)
        .map(|p| p.points).sum();
    let (gw_projected_points, projected_points_explanation) = calculate_projected_points(&players);

    let total_points_before_gw = get_total_points_before_gw(endpoints, id);
    let total_points = total_points_before_gw + gw_points;
    let total_projected_points = total_points_before_gw + gw_projected_points;

    TableEntry {
        team_code,
        owner_name,
        team_name,
        total_points,
        total_projected_points,
        gw_points,
        gw_projected_points,
        projected_points_explanation,
        players,
    }
}

fn extract_players(endpoints: &FplEndpoints, team_id: u32) -> Vec<TablePlayer> {
    let mut players = Vec::new();
    let team_entry = endpoints.teams_gws.get(&team_id)
        .expect(&format!("Could not find team GW info from team {}", team_id));
    for pick in team_entry.picks.iter() {
        let player_id = pick.element as u32;

        let id = player_id;
        let full_name = propcomp::get_player_full_name(endpoints, player_id);
        let display_name = propcomp::get_player_display_name(endpoints, player_id);
        let team = propcomp::compute_player_team(endpoints, player_id);
        let team_pos = PlayerPosition::from_number(propcomp::get_player_position(endpoints, player_id));
        let points = propcomp::get_player_points(endpoints, player_id);
        let bps = propcomp::get_player_bps(endpoints, player_id);
        let projected_points = propcomp::get_player_projected_points(endpoints, player_id);
        let point_sources = propcomp::get_player_point_sources(endpoints, player_id);
        let on_field = propcomp::compute_player_is_on_field(pick, team_entry);
        let pick_number = pick.position;
        let has_played = propcomp::compute_player_has_played(endpoints, player_id);
        let fixtures_finished = propcomp::compute_player_fixtures_has_finished(endpoints, player_id);
        let has_upcoming_fixtures = propcomp::compute_player_has_upcoming_fixtures(endpoints, player_id);

        let player = TablePlayer {
            id,
            full_name,
            display_name,
            team,
            team_pos,
            points,
            bps,
            projected_points,
            point_sources,
            on_field,
            pick_number,
            has_played,
            fixtures_finished,
            has_upcoming_fixtures,
        };
        players.push(player);
    }


    players
}


// Calculates the total number of points for the team
fn calculate_projected_points(players: &Vec<TablePlayer>) -> (i32, Vec<ProjectedPointsExplanation>) {
    let mut projected_playing_players: Vec<&TablePlayer> = players.iter()
        .filter(|p| (p.on_field && p.has_played) || (p.on_field && !p.fixtures_finished))
        .collect();

    let mut benched_players: Vec<&TablePlayer> = players.iter()
        .filter(|p| p.pick_number >= 12 && !p.on_field)
        .collect();
    benched_players.sort_by_key(|p| p.pick_number);

    let mut gks: Vec<&TablePlayer> = projected_playing_players.iter()
        .filter(|p| p.team_pos.number == 1)
        .map(|p| *p)
        .collect();
    let mut defs: Vec<&TablePlayer> = projected_playing_players.iter()
        .filter(|p| p.team_pos.number == 2)
        .map(|p| *p)
        .collect();
    let mut mids: Vec<&TablePlayer> = projected_playing_players.iter()
        .filter(|p| p.team_pos.number == 3)
        .map(|p| *p)
        .collect();
    let mut fwds: Vec<&TablePlayer> = projected_playing_players.iter()
        .filter(|p| p.team_pos.number == 4)
        .map(|p| *p)
        .collect();

    let mut subbed_in_players: Vec<&TablePlayer> = Vec::new();

    // Sub in benched players if too few are on pitch, these will always have a space in the team
    // for these players as a standard team requires a certain amount of players at each position
    benched_players.retain(|p| {
        let mut retain = true;
        match p.team_pos.number {
            1 => {
                if gks.len() < 1 {
                    projected_playing_players.push(p);
                    subbed_in_players.push(p);
                    gks.push(p);
                    retain = false;
                }
            }
            2 => {
                if defs.len() < 3 {
                    projected_playing_players.push(p);
                    subbed_in_players.push(p);
                    defs.push(p);
                    retain = false;
                }
            }
            3 => {
                if mids.len() < 2 {
                    projected_playing_players.push(p);
                    subbed_in_players.push(p);
                    mids.push(p);
                    retain = false;
                }
            }
            4 => {
                if fwds.len() < 1 {
                    projected_playing_players.push(p);
                    subbed_in_players.push(p);
                    fwds.push(p);
                    retain = false;
                }
            }
            _ => {}
        }
        retain
    });

    // See if there are any benched players that we can fit in the on field eleven. Now only
    // the team size cap is the issue: 11, except for goalkeepers, they can't both fit and
    // any substitution must have taken place before
    benched_players.iter().for_each(|p| {
        if projected_playing_players.len() < 11 && p.team_pos.number != 1 {

            // Reserve slot in projected playing players benched players either haven't finished
            // their fixture or have played. This excludes players on the bench that haven't and
            // will not play this GW.
            if !p.fixtures_finished || p.has_played {
                projected_playing_players.push(p);
            }
            if p.has_played {
                subbed_in_players.push(p);
            }
        }
    });

    let mut explanations: Vec<ProjectedPointsExplanation> = Vec::new();
    projected_playing_players.iter().for_each(|p| {
        if p.has_played && p.points > 0 {
            let proj_diff = p.projected_points - p.points;
            let bonus_opt = if proj_diff != 0 { Some(proj_diff) } else { None };

            let subbed_pts_opt = if subbed_in_players.iter().any(|p_compare| p.id == p_compare.id) {
                Some(p.points)
            } else {
                None
            };

            if bonus_opt.is_some() || subbed_pts_opt.is_some() {
                let x = ProjectedPointsExplanation {
                    name: p.display_name.clone(),
                    bonus_points: bonus_opt,
                    subbed_points: subbed_pts_opt,
                };
                explanations.push(x);
            }
        }
    });

    (projected_playing_players.iter().map(|p| p.projected_points).sum(), explanations)
}

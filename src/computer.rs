use std::{thread, time};
use std::sync::{Arc, RwLock};

use crate::propcomp;
use crate::storage::{FplEndpoints, LeagueTable};
use crate::storage::table::Entry as TableEntry;
use crate::storage::table::Player as TablePlayer;
use crate::storage::table::Position as PlayerPosition;

#[allow(dead_code)]
pub fn league_table_computer(lock: Arc<RwLock<LeagueTable>>, endpoints_lock: Arc<RwLock<FplEndpoints>>) {
    loop {
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

        {
            let sleep_ms = 10_000;
            log::trace!("Sleeping computer thread for {} ms", sleep_ms);
            thread::sleep(time::Duration::from_millis(sleep_ms));
        }
    }
}

pub fn compute_new_league_table(endpoints: FplEndpoints) -> Option<LeagueTable> {
    let table = LeagueTable {
        entries: compute_league_entries(&endpoints),
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

fn compute_league_entry(endpoints: &FplEndpoints, id: u32) -> TableEntry{
    TableEntry{
        owner_name: propcomp::get_team_owner_name(endpoints, id),
        team_name: propcomp::get_team_name(endpoints, id),
        players: extract_players(endpoints, id),
        points: 0,
        projected_points: 0,
    }
}

fn extract_players(endpoints: &FplEndpoints, team_id: u32) -> Vec<TablePlayer> {
    let mut players = Vec::new();
    let team_entry = endpoints.teams_gws.get(&team_id)
        .expect(&format!("Could not find team GW info from team {}", team_id));
    for pick in  team_entry.picks.iter() {
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
        let has_played = propcomp::compute_player_has_played(endpoints, player_id);
        let fixtures_finished = propcomp::compute_player_fixtures_has_finished(endpoints, player_id);

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
            has_played,
            fixtures_finished,
        };
        players.push(player);
    }


    players
}

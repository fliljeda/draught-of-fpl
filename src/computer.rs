use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::{Arc, RwLock};
use std::{thread, time};

use crate::propcomp;
use crate::storage::table::H2HMatch as TableH2HMatch;
use crate::storage::table::PlayStatus as PlayerPlayStatus;
use crate::storage::table::Player as TablePlayer;
use crate::storage::table::Position;
use crate::storage::table::{Entry as TableEntry, H2HInfo, ProjectedPointsExplanation, Scoring};
use crate::storage::{FplEndpoints, LeagueTable};

#[allow(dead_code)]
pub async fn league_table_computer(
    lock: Arc<RwLock<LeagueTable>>,
    endpoints_lock: Arc<RwLock<FplEndpoints>>,
) {
    loop {
        {
            let sleep_ms = 30_000;
            log::trace!("Sleeping computer thread for {} ms", sleep_ms);
            thread::sleep(time::Duration::from_millis(sleep_ms));
        }

        let endpoints = match endpoints_lock.read() {
            Ok(e) => (*e).clone(),
            Err(e) => {
                log::error!(
                    "Could not grab read lock for endpoints in computer thread, {}",
                    e
                );
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

    let matches = compute_all_league_matches(&endpoints);
    let table = LeagueTable {
        entries,
        code: propcomp::get_league_id(&endpoints),
        name: propcomp::get_league_name(&endpoints),
        scoring: propcomp::get_league_scoring(&endpoints),
        matches,
    };
    Some(table)
}

fn compute_all_league_matches(
    endpoints: &FplEndpoints,
) -> Option<HashMap<u32, Vec<TableH2HMatch>>> {
    match Scoring::from_fpl_str(&endpoints.details.league.scoring) {
        Scoring::CLASSIC => None,
        Scoring::H2H => {
            let mut league_matches: HashMap<u32, Vec<TableH2HMatch>> = HashMap::new();
            let n_gameweeks: u32 = endpoints
                .static_info
                .events
                .data
                .len()
                .try_into()
                .unwrap_or(38);
            for gw in 1..(n_gameweeks + 1) {
                let gw_matches = compute_gw_league_matches(gw, endpoints);
                league_matches.insert(gw, gw_matches);
            }
            Some(league_matches)
        }
    }
}
fn compute_gw_league_matches(gw: u32, endpoints: &FplEndpoints) -> Vec<TableH2HMatch> {
    match &endpoints.details.matches {
        Option::None => Vec::new(),
        Option::Some(league_matches) => league_matches
            .iter()
            .filter(|league_match| league_match.event == gw)
            .map(|league_match| compute_table_league_match(league_match))
            .collect(),
    }
}

fn compute_table_league_match(league_match: &crate::structs::details::H2HMatch) -> TableH2HMatch {
    let gw = league_match.event;
    let league_entry_1 = league_match.league_entry_1;
    let league_entry_2 = league_match.league_entry_2;
    let started = league_match.started;
    let finished = league_match.finished;
    TableH2HMatch {
        gw,
        league_entry_1,
        league_entry_2,
        started,
        finished,
    }
}

fn compute_league_entries(endpoints: &FplEndpoints) -> Vec<TableEntry> {
    let entries: Vec<TableEntry> = endpoints
        .details
        .league_entries
        .iter()
        .map(|entry| compute_league_entry(endpoints, entry.entry_id))
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

    let gw_points = players
        .iter()
        .filter(|p| p.on_field)
        .map(|p| p.points)
        .sum();

    let gw_projected_points = calculate_projected_points(&players);
    let projected_points_explanation = calculate_projected_point_explanation(&players);

    let total_points_before_gw = get_total_points_before_gw(endpoints, id);
    let total_points = total_points_before_gw + gw_points;
    let total_projected_points = total_points_before_gw + gw_projected_points;

    let h2h_info = compute_h2h_info(endpoints, id);

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
        h2h_info,
    }
}

fn compute_h2h_info(endpoints: &FplEndpoints, id: u32) -> Option<H2HInfo> {
    let team_id = propcomp::get_team_id_from_entry_id(endpoints, id);

    let team_standings = match endpoints
        .details
        .standings
        .iter()
        .find(|entry| entry.league_entry == team_id)
    {
        None => {
            return None;
        }
        Some(standings) => standings,
    };

    let points: i32 = team_standings.total;
    let matches_drawn: u32 = team_standings.matches_drawn.unwrap_or(0);
    let matches_lost: u32 = team_standings.matches_lost.unwrap_or(0);
    let matches_won: u32 = team_standings.matches_won.unwrap_or(0);
    let matches_played: u32 = matches_won + matches_drawn + matches_lost;
    let current_opponent: u32 = propcomp::get_current_h2h_opponent(endpoints, id);
    Some(H2HInfo {
        points,
        matches_won,
        matches_played,
        matches_drawn,
        matches_lost,
        current_opponent,
    })
}

fn extract_players(endpoints: &FplEndpoints, team_id: u32) -> Vec<TablePlayer> {
    let mut players = Vec::new();
    let team_entry = endpoints.teams_gws.get(&team_id).expect(&format!(
        "Could not find team GW info from team {}",
        team_id
    ));
    for pick in team_entry.picks.iter() {
        let player_id = pick.element as u32;

        let id = player_id;
        let full_name = propcomp::get_player_full_name(endpoints, player_id);
        let display_name = propcomp::get_player_display_name(endpoints, player_id);
        let team = propcomp::compute_player_team(endpoints, player_id);
        let team_pos = Position::from_number(propcomp::get_player_position(endpoints, player_id));
        let points = propcomp::get_player_points(endpoints, player_id);
        let bps = propcomp::get_player_bps(endpoints, player_id);
        let projected_points = propcomp::get_player_projected_points(endpoints, player_id);
        let point_sources = propcomp::get_player_point_sources(endpoints, player_id);
        let on_field = propcomp::compute_player_is_on_field(pick, team_entry);
        let pick_number = pick.position;
        let has_played = propcomp::compute_player_has_played(endpoints, player_id);
        let fixtures_finished =
            propcomp::compute_player_fixtures_has_finished(endpoints, player_id);
        let has_upcoming_fixtures =
            propcomp::compute_player_has_upcoming_fixtures(endpoints, player_id);
        let news = propcomp::get_player_news(endpoints, player_id);
        let status = propcomp::get_player_injury_status(endpoints, player_id);

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
            news,
            status,
            play_status: PlayerPlayStatus::Unknown,
        };
        players.push(player);
    }
    calculate_play_status(&mut players);

    players
}

// Calculates the total number of points for the team
fn calculate_projected_points(players: &Vec<TablePlayer>) -> i32 {
    players
        .iter()
        .filter(|p| match p.play_status {
            PlayerPlayStatus::Playing | PlayerPlayStatus::SubbedIn { subbed_with: _ } => true,
            _ => false,
        })
        .map(|p| p.projected_points)
        .sum()
}

fn calculate_projected_point_explanation(
    players: &Vec<TablePlayer>,
) -> Vec<ProjectedPointsExplanation> {
    let mut explanations: Vec<ProjectedPointsExplanation> = Vec::new();
    players
        .iter()
        .filter(|p| match p.play_status {
            PlayerPlayStatus::Playing | PlayerPlayStatus::SubbedIn { subbed_with: _ } => true,
            _ => false,
        })
        .for_each(|p| {
            if p.has_played && p.points > 0 {
                let proj_diff = p.projected_points - p.points;
                let bonus_opt = if proj_diff != 0 {
                    Some(proj_diff)
                } else {
                    None
                };

                let subbed_pts_opt =
                    if let PlayerPlayStatus::SubbedIn { subbed_with: _ } = p.play_status {
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
    explanations
}

fn calculate_play_status(players: &mut Vec<TablePlayer>) {
    // Sort players by pick_number to ensure bench order
    players.sort_by(|p1, p2| p1.pick_number.cmp(&p2.pick_number));

    struct Substitution {
        player_in: u32,
        player_out: u32,
    }

    let mut subs: Vec<Substitution> = Vec::new();

    // First pass: Set initial statuses
    for player in players.iter_mut() {
        if player.on_field && (player.has_played || !player.fixtures_finished) {
            player.play_status = PlayerPlayStatus::Playing;
        }
        if !player.on_field {
            player.play_status = PlayerPlayStatus::Benched;
        }
    }

    // Handle goalkeeper substitutions first
    for player in players
        .iter()
        .filter(|p| p.on_field && p.team_pos == Position::GK)
    {
        if !player.has_played && player.fixtures_finished {
            if let Some(other_gk) = players
                .iter()
                .find(|p| p.team_pos == Position::GK && p.id != player.id)
            {
                if other_gk.has_played || other_gk.has_upcoming_fixtures {
                    subs.push(Substitution {
                        player_in: other_gk.id,
                        player_out: player.id,
                    });
                }
            }
        }
    }

    // First priority: Ensure minimum position requirements
    let position_requirements = [(Position::DEF, 3), (Position::MID, 2), (Position::FWD, 1)]; // (position, min_required)

    for (pos, min_required) in position_requirements.iter() {
        // Check if we need more players in this position
        let current_playing = players
            .iter()
            .filter(|p| {
                p.team_pos == *pos
                    && (p.play_status == PlayerPlayStatus::Playing
                        || matches!(p.play_status, PlayerPlayStatus::SubbedIn { .. }))
            })
            .count();

        if current_playing < *min_required {
            // Find non-playing players in this position
            let non_playing = players
                .iter()
                .filter(|p| {
                    p.on_field && p.team_pos == *pos && !p.has_played && p.fixtures_finished
                })
                .collect::<Vec<_>>();

            // Find available bench players in this position
            let bench_players = players
                .iter()
                .filter(|p| {
                    !p.on_field
                        && p.team_pos == *pos
                        && (p.has_played || !p.fixtures_finished)
                        && !subs
                            .iter()
                            .any(|s| s.player_in == p.id || s.player_out == p.id)
                })
                .collect::<Vec<_>>();

            // Make necessary substitutions
            for (i, non_playing_player) in non_playing
                .iter()
                .take(*min_required - current_playing)
                .enumerate()
            {
                if let Some(bench_player) = bench_players.get(i) {
                    subs.push(Substitution {
                        player_in: bench_player.id,
                        player_out: non_playing_player.id,
                    });
                }
            }
        }
    }

    // Second priority: Fill remaining spots up to 11 players
    // Only process this after minimum position requirements are met
    let playing_outfield_count = players
        .iter()
        .filter(|p| {
            p.team_pos != Position::GK
                && (p.play_status == PlayerPlayStatus::Playing
                    || matches!(p.play_status, PlayerPlayStatus::SubbedIn { .. }))
        })
        .count();

    if playing_outfield_count < 10 {
        // collect candidate IDs first so we don't borrow `subs` inside iterator closures
        let benched_candidates: Vec<u32> = players
            .iter()
            .filter(|p| {
                let not_already_in_sub_list = !subs
                    .iter()
                    .any(|s| s.player_in == p.id || s.player_out == p.id);
                let played_or_will_play = p.has_played || !p.fixtures_finished;

                p.selected_on_bench()
                    && !p.on_field
                    && p.team_pos != Position::GK
                    && played_or_will_play
                    && not_already_in_sub_list
            })
            .map(|p| p.id)
            .collect();

        // collect non-playing on-field player IDs that could be subbed off
        let mut available_non_playing_ids: Vec<u32> = players
            .iter()
            .filter(|p| {
                p.on_field
                    && !p.has_played
                    && p.fixtures_finished
                    && !subs.iter().any(|s| s.player_out == p.id)
            })
            .map(|p| p.id)
            .collect();

        for benched_id in benched_candidates {
            // skip if this benched player is already involved in a planned sub
            if subs
                .iter()
                .any(|s| s.player_in == benched_id || s.player_out == benched_id)
            {
                continue;
            }

            // find a non-playing player not already targeted by a sub
            if let Some(idx) = available_non_playing_ids
                .iter()
                .position(|np_id| !subs.iter().any(|s| s.player_out == *np_id))
            {
                let non_playing_id = available_non_playing_ids.remove(idx);
                subs.push(Substitution {
                    player_in: benched_id,
                    player_out: non_playing_id,
                });
            } else {
                // no suitable non-playing player left
                break;
            }
        }
    }

    // Apply all substitutions
    for sub in subs {
        if let Some(p) = players.iter_mut().find(|p| p.id == sub.player_in) {
            p.play_status = PlayerPlayStatus::SubbedIn {
                subbed_with: sub.player_out,
            };
        }
        if let Some(p) = players.iter_mut().find(|p| p.id == sub.player_out) {
            p.play_status = PlayerPlayStatus::SubbedOff {
                subbed_with: sub.player_in,
            };
        }
    }

    // Set remaining unknown statuses
    for p in players
        .iter_mut()
        .filter(|p| p.play_status == PlayerPlayStatus::Unknown)
    {
        p.play_status = PlayerPlayStatus::Playing;
    }
}

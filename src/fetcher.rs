use std::{thread, time};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use futures::join;

use crate::client::{Client, ClientError};
use crate::storage::FplEndpoints;
use crate::structs::*;
use std::ops::Deref;

#[allow(dead_code)]
pub fn endpoint_cache_fetcher(endpoints_lock: Arc<RwLock<FplEndpoints>>, client: Client, context_lock: Arc<RwLock<crate::AppContext>>) {
    loop {
        let app_context = match context_lock.read() {
            Ok(c) => {
                (*c.deref()).clone()
            },
            Err(_) => {
                continue;
            }
        };
        let fetch_sleep_ms = app_context.fetch_sleep_ms;
        let new: FplEndpoints = fetch_new_endpoints(&client, app_context);
        match endpoints_lock.write() {
            Ok(mut t) => {
                println!("Grabbed the lock");
                t.update(new);
            }
            Err(e) => {
                println!("Could not grab write lock for table: {}", e);
            }
        };
        {
            log::info!("Sleeping for {} ms", fetch_sleep_ms);
            thread::sleep(time::Duration::from_millis(fetch_sleep_ms));
        }
    }
}

fn handle_error_into_option<T>(res: Result<T, ClientError>) -> Option<T> {
    let the_type = std::any::type_name::<T>();
    return match res {
        Ok(x) => Some(x),
        Err(e) => {
            log::error!("Error retrieving {}: {}", the_type, e);
            None
        }
    };
}

fn fetch_new_endpoints(client: &Client, context: crate::AppContext) -> FplEndpoints {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let league_code = context.league_id;
        let gw: u32 = context.gw;
        let teams = context.team_ids;

        let (game, details) = fetch_game_and_details_with_retries(client, &league_code).await;


        // Start http_calls
        let live = client.get_gw_points_live(&gw);
        let static_info = client.get_static();
        let team_gws_res = client.get_multiple_teams_gw(&teams, &gw);
        let team_infos_res = client.get_multiple_teams_info(&teams);

        // Handle results when returned
        let live = handle_error_into_option(live.await);
        let static_info = handle_error_into_option(static_info.await);
        let team_gws_res = team_gws_res.await;
        let team_infos_res = team_infos_res.await;

        // Convert hashmaps to use options in finalized result
        let mut team_gws: HashMap<u32, Option<TeamGw>> = HashMap::new();
        for (team, res) in team_gws_res.into_iter() {
            team_gws.insert(team, handle_error_into_option(res));
        }

        let mut team_infos: HashMap<u32, Option<TeamInfo>> = HashMap::new();
        for (team, res) in team_infos_res.into_iter() {
            team_infos.insert(team, handle_error_into_option(res));
        }

        FplEndpoints {
            details,
            static_info,
            game,
            teams_infos: team_infos,
            teams_gws: team_gws,
            live,
        }
    })
}

// Fetches the crucial endpoints from FPL Api with retry
pub async fn fetch_game_and_details_with_retries(client: &Client, league_code: &u32) -> (Option<Game>, Option<Details>) {
    let mut retries = 15;
    let retry_wait_ms: u64 = 1000;
    while retries > 0 {
        let details = client.get_league_details(&league_code);
        let game = client.get_game();
        let (game, details): (Result<Game, ClientError>, Result<Details, ClientError>) = join!(game, details);
        match (game, details) {
            (Ok(g), Ok(d)) => {
                return (Some(g), Some(d));
            }
            (Ok(g), Err(e)) => {
                let d = fetch_details_with_retries(client, &mut retries, league_code, retry_wait_ms).await;
                log::error!("Error fetching Details. \nDetails: {}", e);
                return (Some(g), d);
            }
            (Err(e), Ok(d)) => {
                let g = fetch_game_with_retries(client, &mut retries, retry_wait_ms).await;
                log::error!("Error fetching Game. \nGame: {}", e);
                return (g, Some(d));
            }
            (Err(eg), Err(ed)) => {
                log::error!("Error fetching both Game and Details \nGame: {}\nDetails: {}", eg, ed);
                retries -= 1;
            }
        }
        thread::sleep(time::Duration::from_millis(retry_wait_ms));
    }
    (None, None)
}

async fn fetch_game_with_retries(client: &Client, retries: &mut i32, retry_wait_ms: u64) -> Option<Game> {
    while *retries > 0 {
        let details = client.get_game().await;
        match details {
            Ok(g) => {
                return Some(g);
            }
            Err(e) => {
                log::error!("Error fetching Game. \nGame: {}", e);
                *retries -= 1;
            }
        }
        thread::sleep(time::Duration::from_millis(retry_wait_ms));
    }
    return None;
}

async fn fetch_details_with_retries(client: &Client, retries: &mut i32, league_code: &u32, retry_wait_ms: u64) -> Option<Details> {
    while *retries > 0 {
        let details = client.get_league_details(&league_code).await;
        match details {
            Ok(d) => {
                return Some(d);
            }
            Err(e) => {
                log::error!("Error fetching Details. \nDetails: {}", e);
                *retries -= 1;
            }
        }
        thread::sleep(time::Duration::from_millis(retry_wait_ms));
    }
    return None;
}



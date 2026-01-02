use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::{thread, time};

use futures::join;

use crate::client::{Client, ClientError};
use crate::storage::endpoints::FplEndpointsUpdate;
use crate::storage::FplEndpoints;
use crate::structs::*;

#[allow(dead_code)]
pub async fn endpoint_cache_fetcher(
    client: Client,
    endpoints_lock: Arc<RwLock<FplEndpoints>>,
    context: Arc<crate::AppContext>,
) {
    let mut static_info_last_fetch: Option<time::Instant> = None;
    loop {
        let app_context = context.deref().clone();
        let fetch_sleep_duration = app_context.fetch_sleep_duration;

        {
            tracing::trace!(
                "Sleeping fetcher thread for {} ms",
                fetch_sleep_duration.as_millis()
            );
            thread::sleep(fetch_sleep_duration);
        }

        tracing::debug!("Fetching new endpoints");
        let new = fetch_new_endpoints(&client, app_context, &mut static_info_last_fetch).await;
        match endpoints_lock.write() {
            Ok(mut t) => {
                tracing::trace!("Grabbed the lock");
                t.update(new);
            }
            Err(e) => {
                tracing::error!("Could not grab write lock for endpoints: {}", e);
            }
        };
    }
}

fn handle_error_into_option<T>(res: Result<T, ClientError>) -> Option<T> {
    let the_type = std::any::type_name::<T>();
    return match res {
        Ok(x) => Some(x),
        Err(e) => {
            tracing::error!("Error retrieving {}: {}", the_type, e);
            None
        }
    };
}

pub async fn fetch_and_initialize_endpoints(
    client: &Client,
    context: crate::AppContext,
) -> FplEndpoints {
    let mut last_fetch: Option<time::Instant> = None;
    let endpoints = fetch_new_endpoints(&client, context, &mut last_fetch).await;
    let endpoints = FplEndpoints::initialize_from_update(endpoints);
    endpoints
}

pub async fn fetch_new_endpoints(
    client: &Client,
    context: crate::AppContext,
    static_info_last_fetch: &mut Option<time::Instant>,
) -> FplEndpointsUpdate {
    let mut gw = 1;
    let league_code = context.league_id;
    let teams = context.team_ids;

    //let (game, details) = fetch_game_and_details_with_retries(client, &league_code).await;
    let retries = 15;
    let retry_delay_ms = 10;
    let game = fetch_game_with_retries(client, retries, retry_delay_ms);
    let details = fetch_details_with_retries(client, retries, &league_code, retry_delay_ms);
    let (game, details): (Option<Game>, Option<Details>) = join!(game, details);

    game.as_ref().map(|game| match game.current_event {
        Some(current_gw) => {
            gw = current_gw;
        }
        None => {
            tracing::error!("Did not find new GW in fetch, using GW: {}", gw);
        }
    });

    // Start http_calls
    let live = client.get_gw_points_live(&gw);
    let team_gws_res = client.get_multiple_teams_gw(&teams, &gw);
    let team_infos_res = client.get_multiple_teams_info(&teams);

    // Handle static because we don't need to update the endpoint too often
    let static_info = match static_info_last_fetch {
        Some(i) if i.elapsed() < context.static_info_fetch_freq => None,
        _ => {
            tracing::debug!("Fetching static info");
            *static_info_last_fetch = Some(time::Instant::now());
            handle_error_into_option(client.get_static().await)
        }
    };

    // Handle results when returned
    let live = handle_error_into_option(live.await);
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

    FplEndpointsUpdate {
        details,
        static_info,
        game,
        teams_infos: team_infos,
        teams_gws: team_gws,
        live,
    }
}

async fn fetch_game_with_retries(
    client: &Client,
    mut retries: i32,
    retry_wait_ms: u64,
) -> Option<Game> {
    let mut err: Option<ClientError> = None;
    while retries > 0 {
        let details = client.get_game().await;
        match details {
            Ok(g) => {
                return Some(g);
            }
            Err(e) => {
                err = Some(e);
                retries -= 1;
            }
        }
        thread::sleep(time::Duration::from_millis(retry_wait_ms));
    }
    if let Some(e) = err {
        tracing::error!("Error fetching Game. \nGame: {}", e);
    }
    return None;
}

async fn fetch_details_with_retries(
    client: &Client,
    mut retries: i32,
    league_code: &u32,
    retry_wait_ms: u64,
) -> Option<Details> {
    let mut err: Option<ClientError> = None;
    while retries > 0 {
        let details = client.get_league_details(&league_code).await;
        match details {
            Ok(d) => {
                return Some(d);
            }
            Err(e) => {
                err = Some(e);
                retries -= 1;
            }
        }
        thread::sleep(time::Duration::from_millis(retry_wait_ms));
    }
    if let Some(e) = err {
        tracing::error!("Error fetching Details. \nDetails: {}", e);
    }
    return None;
}

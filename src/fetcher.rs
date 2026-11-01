use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time;

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
            tokio::time::sleep(fetch_sleep_duration).await;
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
    match res {
        Ok(x) => Some(x),
        Err(e) => {
            tracing::error!("Error retrieving {}: {}", the_type, e);
            None
        }
    }
}

pub async fn fetch_and_initialize_endpoints(
    client: &Client,
    context: crate::AppContext,
) -> FplEndpoints {
    let mut last_fetch: Option<time::Instant> = None;
    let endpoints = fetch_new_endpoints(client, context, &mut last_fetch).await;

    FplEndpoints::initialize_from_update(endpoints)
}

pub async fn fetch_new_endpoints(
    client: &Client,
    context: crate::AppContext,
    static_info_last_fetch: &mut Option<time::Instant>,
) -> FplEndpointsUpdate {
    let league_code = context.league_id;
    let teams = context.team_ids;

    //let (game, details) = fetch_game_and_details_with_retries(client, &league_code).await;
    let retries = 15;
    let retry_delay_ms = 10;
    let game = fetch_game_with_retries(client, retries, retry_delay_ms);
    let details = fetch_details_with_retries(client, retries, &league_code, retry_delay_ms);
    let (game, details): (Option<Game>, Option<Details>) = join!(game, details);

    // Without the current GW we don't know which live data and picks to fetch. Skipping them
    // keeps the ones we already have.
    let gw = game.as_ref().and_then(|game| game.current_event);
    if gw.is_none() {
        tracing::error!("Did not find the current GW in fetch, skipping live data and picks");
    }

    // Start http_calls
    let team_infos_res = client.get_multiple_teams_info(&teams);

    // Handle static because we don't need to update the endpoint too often
    let static_info = match static_info_last_fetch {
        Some(i) if i.elapsed() < context.static_info_fetch_freq => None,
        _ => {
            tracing::debug!("Fetching static info");
            let static_info = handle_error_into_option(client.get_static().await);
            // Only a successful fetch waits for the next refresh. A failed one retries next cycle.
            if static_info.is_some() {
                *static_info_last_fetch = Some(time::Instant::now());
            }
            static_info
        }
    };

    // Handle results when returned
    let live = match gw {
        Some(gw) => handle_error_into_option(client.get_gw_points_live(&gw).await),
        None => None,
    };
    let team_gws_res = match gw {
        Some(gw) => client.get_multiple_teams_gw(&teams, &gw).await,
        None => HashMap::new(),
    };
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
        tokio::time::sleep(time::Duration::from_millis(retry_wait_ms)).await;
    }
    if let Some(e) = err {
        tracing::error!("Error fetching Game. \nGame: {}", e);
    }
    None
}

async fn fetch_details_with_retries(
    client: &Client,
    mut retries: i32,
    league_code: &u32,
    retry_wait_ms: u64,
) -> Option<Details> {
    let mut err: Option<ClientError> = None;
    while retries > 0 {
        let details = client.get_league_details(league_code).await;
        match details {
            Ok(d) => {
                return Some(d);
            }
            Err(e) => {
                err = Some(e);
                retries -= 1;
            }
        }
        tokio::time::sleep(time::Duration::from_millis(retry_wait_ms)).await;
    }
    if let Some(e) = err {
        tracing::error!("Error fetching Details. \nDetails: {}", e);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn test_context() -> crate::AppContext {
        crate::AppContext {
            league_id: 26952,
            team_ids: vec![138589],
            fetch_sleep_duration: Duration::from_secs(60),
            static_info_fetch_freq: Duration::from_secs(1800),
        }
    }

    #[tokio::test]
    async fn failed_game_fetch_does_not_try_to_fetch_picks() {
        // Every fetch fails, because this folder does not exist
        let client = Client::new_local(Some(String::from("does-not-exist"))).unwrap();

        let update = fetch_new_endpoints(&client, test_context(), &mut None).await;

        assert!(update.game.is_none());
        // A fallback to GW 1 would leave a failed entry for this team here
        assert!(update.teams_gws.is_empty());
    }

    #[tokio::test]
    async fn failed_static_fetch_does_not_count_as_the_last_fetch() {
        // Every fetch fails, because this folder does not exist
        let client = Client::new_local(Some(String::from("does-not-exist"))).unwrap();
        let mut last_fetch = None;

        fetch_new_endpoints(&client, test_context(), &mut last_fetch).await;

        assert!(last_fetch.is_none());
    }

    #[tokio::test]
    async fn successful_static_fetch_counts_as_the_last_fetch() {
        let api = format!(
            "{}/tests/snapshots/2026-10-31-gw8-finished-gw/api",
            env!("CARGO_MANIFEST_DIR")
        );
        let client = Client::new_local(Some(api)).unwrap();
        let mut last_fetch = None;

        fetch_new_endpoints(&client, test_context(), &mut last_fetch).await;

        assert!(last_fetch.is_some());
    }
}

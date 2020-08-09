#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::{Arc, RwLock};

use rocket::State;

use crate::storage::FplEndpoints;
use crate::client::Client;

mod client;
mod fetcher;
mod storage;
mod structs;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub league_id: u32,
    pub team_ids: Vec<u32>,
    pub gw: u32,
    pub fetch_sleep_ms: u64,
}

async fn initialize_app_context(client: &Client) -> AppContext {
    let league_id= 305;
    let game = client.get_game().await.unwrap();
    let details = client.get_league_details(&league_id).await.unwrap();
    let fetch_sleep_ms = 30_000 as u64;
    AppContext{
        league_id,
        team_ids: details.league_entries.iter().map(|x| x.entry_id).collect(),
        gw: game.current_event,
        fetch_sleep_ms,
    }
}


#[tokio::main]
pub async fn main() {
    let client = Client::new().unwrap();
    let app_context = Arc::new(RwLock::new(initialize_app_context(&client).await));
    let app_context_clone = Arc::clone(&app_context);

    let endpoints = Arc::new(RwLock::new(FplEndpoints::create_blank()));
    let endpoints_clone = Arc::clone(&endpoints);

    std::thread::spawn(|| fetcher::endpoint_cache_fetcher(endpoints_clone, client, app_context_clone));

    let rocket = rocket::ignite()
        .mount("/fpl", routes![get_player])
        .manage(endpoints);

    rocket.launch();
}


#[get("/player/<id>")]
fn get_player(id: u32, endpoints: State<Arc<RwLock<FplEndpoints>>>) -> String {
    return match endpoints.read() {
        Ok(ep) => {
            if let Some(static_info) = &ep.static_info {
                let i= (id - 1) as usize;
                return match static_info.elements.get(i) {
                    Some(element) => format!("Player: {} {} with id {}\n", element.first_name, element.second_name, element.id),
                    None => format!("Could not find player with id {}", id),
                };
            }
            format!("Have not retrieved static info")
        },
        Err(_e) => {
            format!("Error reading endpoints")
        }
    }
}

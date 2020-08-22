#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::{Arc, RwLock};

use rocket::State;

use crate::storage::FplEndpoints;
use crate::client::Client;
pub use initializer::AppContext;

mod client;
mod fetcher;
mod storage;
mod structs;
mod initializer;




#[tokio::main]
pub async fn main() {

    let app_config = initializer::AppConfig::initialize();

    let client = match app_config.local_fetch {
        Some(true) => Client::new_local().unwrap(),
        Some(false) | None => Client::new().unwrap(),
    };
    let league_id = app_config.league_id;

    let app_context = Arc::new(RwLock::new(initializer::initialize_app_context(&client, league_id).await));
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
                    Some(element) => {
                        let first_name = element.first_name.as_ref().unwrap();
                        let second_name = element.second_name.as_ref().unwrap();
                        let player_id = element.id.unwrap();
                        format!("Player: {} {} with id {}\n", first_name, second_name, player_id)
                    },
                    None => format!("Could not find player with id {}\n", id),
                };
            }
            format!("Have not retrieved static info")
        },
        Err(_e) => {
            format!("Error reading endpoints")
        }
    }
}

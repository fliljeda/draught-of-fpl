
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::{Arc, RwLock};

use rocket::State;

use crate::storage::FplEndpoints;
use crate::client::Client;
pub use initializer::AppContext;

mod client;
mod propcomp;
mod fetcher;
mod storage;
mod structs;
mod initializer;
mod computer;




#[tokio::main]
pub async fn main() {

    let app_config = initializer::AppConfig::initialize();

    let client = match app_config.local_fetch {
        Some(true) => Client::new_local(app_config.local_url.clone()).unwrap(),
        Some(false) | None => Client::new().unwrap(),
    };

    let league_id = app_config.league_id;

    let app_context = Arc::new(initializer::initialize_app_context(&client, league_id).await);

    let endpoints = Arc::new(RwLock::new(FplEndpoints::create_blank()));
    let endpoints_clone = Arc::clone(&endpoints);

    std::thread::spawn(|| fetcher::endpoint_cache_fetcher(client, endpoints_clone, app_context));

    let rocket = rocket::ignite()
        .mount("/fpl", routes![get_player])
        .mount("/ns", routes![get_table])
        .manage(endpoints);

    rocket.launch();
}


#[get("/player/<id>")]
fn get_player(id: u32, endpoints: State<Arc<RwLock<FplEndpoints>>>) -> String {
    return match endpoints.read() {
        Ok(ep) => {
            let full_name = propcomp::get_player_full_name(&*ep, id);
            format!("Player: {} with id {}\n", full_name, id)
        },
        Err(_e) => {
            format!("Error reading endpoints")
        }
    }
}
#[get("/table")]
fn get_table(endpoints: State<Arc<RwLock<FplEndpoints>>>) -> String {
    return match endpoints.read() {
        Ok(_ep) => {
            format!("Ok")
        },
        Err(_e) => {
            format!("Error reading endpoints")
        }
    }
}

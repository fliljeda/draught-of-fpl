#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::io::Cursor;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use rocket::State;

pub use initializer::AppContext;

use crate::client::Client;
use crate::storage::{
    FplEndpoints,
    LeagueTable,
};
use crate::client::Client;
pub use initializer::AppContext;
use std::ops::Deref;
use std::io::Cursor;

mod client;
mod propcomp;
mod fetcher;
mod storage;
mod structs;
mod initializer;
mod computer;


pub fn main() {
    let app_config = initializer::AppConfig::initialize();

    let client = match app_config.local_fetch {
        Some(true) => Client::new_local(app_config.local_url.clone()).unwrap(),
        Some(false) | None => Client::new().unwrap(),
    };

    let league_id = app_config.league_id;

    let app_context = Arc::new(initializer::initialize_app_context(&client, league_id));

    let endpoints = fetcher::fetch_new_endpoints(&client, app_context.deref().clone());
    let endpoints = storage::FplEndpoints::initialize_from_update(endpoints);

    let initialize_table_endpoints = endpoints.clone();
    let table = computer::compute_new_league_table(initialize_table_endpoints).unwrap();

    let endpoints = Arc::new(RwLock::new(endpoints));
    let endpoints_fetch_clone = Arc::clone(&endpoints);
    let endpoints_compute_clone = Arc::clone(&endpoints);

    let table = Arc::new(RwLock::new(table));
    let table_compute_clone = Arc::clone(&table);

    std::thread::spawn(|| fetcher::endpoint_cache_fetcher(client, endpoints_fetch_clone, app_context));
    std::thread::spawn(|| computer::league_table_computer(table_compute_clone, endpoints_compute_clone));

    let rocket = rocket::ignite()
        .mount("/fpl", routes![get_player])
        .mount("/", routes![get_table])
        .manage(endpoints)
        .manage(table);

    rocket.launch();
}


#[get("/player/<id>")]
fn get_player(id: u32, endpoints: State<Arc<RwLock<FplEndpoints>>>) -> String {
    return match endpoints.read() {
        Ok(ep) => {
            let full_name = propcomp::get_player_full_name(&*ep, id);
            format!("Player: {} with id {}\n", full_name, id)
        }
        Err(_e) => {
            format!("Error reading endpoints")
        }
    };
}

#[get("/table")]
fn get_table(table: State<Arc<RwLock<LeagueTable>>>) -> rocket::Response {
    return match table.read() {
        Ok(t) => {
            let table_ser = serde_json::to_string(t.deref())
                .expect("Could not serialize table");
            to_response(table_ser)
        }
        Err(_e) => {
            to_response(format!("Error reading league table"))
        }
    };
}

fn to_response(content: String) -> rocket::Response<'static> {
    rocket::Response::build()
        .header(rocket::http::ContentType::JSON)
        .raw_header("Access-Control-Allow-Origin", "*")
        .sized_body(Cursor::new(content))
        .finalize()
}
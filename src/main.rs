#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::{Arc, RwLock};

use log::{info, warn};
use rocket::State;

use storage::Entry;
use storage::Table;

mod client;
mod fetcher;
mod storage;
mod structs;


pub fn main() {
    let _table_lock = Arc::new(RwLock::new(Table {
        entries: vec![
            Entry {
                name: String::from("Alltid Redo"),
                points: 1,
            },
            Entry {
                name: String::from("Lag 2"),
                points: 2
            },
        ],
    }));

    let _table_clone = Arc::clone(&_table_lock);

    let rocket = rocket::ignite()
        .mount("/fpl", routes![get_player])
        .mount("/table", routes![get_table])
        .manage(_table_lock);

    rocket.launch();
}

#[get("/player/<id>")]
fn get_player(id: u32) -> String {
    format!("Submitted player ID: {}\n", id)
}

#[get("/")]
fn get_table(table_lock: State<Arc<RwLock<Table>>>) -> String {
    match table_lock.read() {
        Ok(table) => {
            info!("Successful request to table!");
            format!("{:?}", table)
        },
        Err(e) => {
            warn!("Could not grab read lock for table: {}", e);
            String::from("Error accessing the table resource for reading")
        }
    }
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod client;
mod fetcher;
mod storage;
mod structs;

use rocket::State;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use storage::Entry;
use storage::Table;

pub fn main() {
    // Atomic reference counter because the mutex exists in main, fetcher and rocket.
    let mut table = Arc::new(Mutex::new(Table {
        entries: vec![
            Entry {
                name: String::from("Alltid Redo"),
                points: 1,
            },
            Entry {
                name: String::from("Lag 2"),
                points: 2,
            },
        ],
    }));

    spawn(move || fetcher::test(Arc::clone(&table)));

    let rocket = rocket::ignite()
        .mount("/fpl", routes![get_player])
        .mount("/table", routes![get_table])
        .manage(table)
        .launch();

    println!("Hej");
}

#[get("/player/<id>")]
fn get_player(id: u32) -> String {
    format!("Submitted player ID: {}\n", id)
}

#[get("/")]
fn get_table(table: State<Arc<Mutex<Table>>>) -> String {
    format!("{:?}", table.lock().unwrap())
}

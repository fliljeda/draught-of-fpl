#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod client;
mod fetcher;
mod storage;
mod structs;

use rocket::State;
use std::sync::{atomic::AtomicI32, Arc};
use std::thread::spawn;
use storage::Entry;
use storage::Table;

pub fn main() {
    // Atomic reference counter because the mutex exists in main, fetcher and rocket.
    let mut table = Arc::new(Table {
        entries: vec![
            Entry {
                name: String::from("Alltid Redo"),
                points: AtomicI32::new(1),
            },
            Entry {
                name: String::from("Lag 2"),
                points: AtomicI32::new(2),
            },
        ],
    });

    let table_clone = Arc::clone(&table);
    spawn(move || fetcher::test(table_clone));

    let rocket = rocket::ignite()
        .mount("/fpl", routes![get_player])
        .mount("/table", routes![get_table])
        .manage(table);

    rocket.launch();
}

#[get("/player/<id>")]
fn get_player(id: u32) -> String {
    format!("Submitted player ID: {}\n", id)
}

#[get("/")]
fn get_table(table: State<Arc<Table>>) -> String {
    format!("{:?}", table)
}

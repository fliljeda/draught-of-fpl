use std::{thread, time};
use std::sync::{Arc, RwLock, RwLockReadGuard};


use crate::propcomp;
use crate::storage::{FplEndpoints, LeagueTable};

#[allow(dead_code)]
pub fn league_table_computer(lock: Arc<RwLock<LeagueTable>>, endpoints_lock: Arc<RwLock<FplEndpoints>>) {
    loop {
        let endpoints = match endpoints_lock.read() {
            Ok(e) => e,
            Err(e) => {
                log::error!("Could not grab read lock for endpoints in computer, {}", e);
                continue;
            }
        };
        let new = compute_new_league_table(endpoints);
        match lock.write() {
            Ok(mut t) => {
                log::trace!("Grabbed the league table lock");
                *t = new.unwrap();
            }
            Err(e) => {
                log::error!("Could not grab write lock for table: {}", e);
            }
        };
        {
            let fetch_sleep_ms = 30_000;
            log::trace!("Sleeping computer thread for {} ms", fetch_sleep_ms);
            thread::sleep(time::Duration::from_millis(fetch_sleep_ms));
        }
    }
}

fn compute_new_league_table(endpoints: RwLockReadGuard<FplEndpoints>) -> Option<LeagueTable> {
    None
}
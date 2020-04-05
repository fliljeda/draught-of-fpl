use crate::storage::Table;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn test(table: Arc<Mutex<Table>>) {
    loop {
        sleep(Duration::from_millis(200));
        table.lock().unwrap().entries[0].points += 1;
    }
}

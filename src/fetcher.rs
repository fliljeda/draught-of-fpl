use crate::storage::Table;
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn test(table: Arc<Table>) {
    loop {
        sleep(Duration::from_millis(200));
        let x = table.entries[0].points.load(Ordering::Relaxed);
        table.entries[0].points.store(x + 1, Ordering::Relaxed);
    }
}

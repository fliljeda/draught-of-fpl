use std::sync::{Arc, RwLock};
use log::warn;

use crate::client::Client;
use crate::storage::FplEndpoints;

// Continuously updates the endpoints
#[allow(dead_code)]
pub fn endpoint_cache_fetcher(endpoints_lock: Arc<RwLock<FplEndpoints>>, _client: &Client) {
    match endpoints_lock.write() {
        Ok(mut _t) => {
            info!("Grabbed the lock")
        }
        Err(e) => {
            warn!("Could not grab write lock for table: {}", e);
        }
    };
}


use std::env;
use std::fs;

use serde::Deserialize;

use crate::client::Client;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub league_id: u32,
    pub team_ids: Vec<u32>,
    pub fetch_sleep_ms: u64,
    pub static_info_fetch_freq_ms: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub league_id: u32,
    pub local_fetch: Option<bool>,
    pub local_url: Option<String>,
}

impl AppConfig {
    /// Initialize AppConfig from file if provided, otherwise fall back to environment variables
    pub fn initialize(config_file_path: Option<String>) -> AppConfig {
        match config_file_path {
            Some(path) => Self::initialize_from_file(path),
            None => Self::initialize_from_env(),
        }
    }

    fn initialize_from_file(config_file_path: String) -> AppConfig {
        let config_file_content = fs::read_to_string(&config_file_path)
            .expect(format!("Failed to read config file: {}", config_file_path).as_str());
        let config: AppConfig = toml::from_str(config_file_content.as_str())
            .expect("Could not parse application config file");
        config
    }

    fn initialize_from_env() -> AppConfig {
        let league_id_str =
            env::var("DOF_LEAGUE_ID").expect("LEAGUE_ID must be set in environment");
        let league_id: u32 = league_id_str
            .parse()
            .expect("DOF_LEAGUE_ID in environment must be a valid u32");

        let local_fetch = match env::var("DOF_LOCAL_FETCH") {
            Ok(val) => match val.as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            },
            Err(_) => None,
        };

        let local_url = match env::var("DOF_LOCAL_URL") {
            Ok(val) => Some(val),
            Err(_) => None,
        };

        AppConfig {
            league_id,
            local_fetch,
            local_url,
        }
    }
}

pub async fn initialize_app_context(client: &Client, league_id: u32) -> AppContext {
    let game = client.get_game().await.unwrap();
    game.current_event
        .expect("No game week found when initializing appContext, must be preseason!");
    let details = client
        .get_league_details(&league_id)
        .await
        .expect("Something went wrong with parsing league details in initialization");

    let team_ids = details.league_entries.iter().map(|x| x.entry_id).collect();

    let fetch_sleep_ms = 60_000 as u64;

    let static_info_fetch_freq_ms = 1_800_000 as u64;

    AppContext {
        league_id,
        team_ids,
        fetch_sleep_ms,
        static_info_fetch_freq_ms,
    }
}

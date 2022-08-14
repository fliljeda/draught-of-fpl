use std::env;
use std::fs;

use serde::Deserialize;

use crate::client::Client;

const CONFIG_FILE_DEFAULT_PATH: &str = "./Config.toml";


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
    pub fn initialize() -> AppConfig {
        let config_file_content = read_config_file();
        let config: AppConfig = toml::from_str(config_file_content.as_str()).expect("Could not parse application config file");
        config
    }
}


fn read_config_file() -> String {
    let config_file_path = read_config_file_flag()
        .unwrap_or(String::from(CONFIG_FILE_DEFAULT_PATH));
    let content = fs::read_to_string(&config_file_path)
        .expect(format!("Failed to read config file: {}", config_file_path).as_str());
    content
}

fn read_config_file_flag() -> Option<String> {
    let mut args_iter = env::args();
    while let Some(arg) = args_iter.next() {
        if arg == "-f" {
            match args_iter.next() {
                Some(p) => {
                    return Some(p);
                },
                None => {
                    panic!("Config file path must follow flag -f")
                }
            }
        }
    }
    None
}


pub async fn initialize_app_context(client: &Client, league_id: u32) -> AppContext {
    let game = client.get_game().await.unwrap();
    game.current_event.expect("No game week found when initializing appContext, must be preseason!");
    let details = client.get_league_details(&league_id).await.expect("Something went wrong with parsing league details in initialization");

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

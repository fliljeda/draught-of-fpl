use std::env;
use std::fs;

use serde::Deserialize;

use crate::client::Client;

const CONFIG_FILE_DEFAULT_PATH: &str = "./config.toml";


#[derive(Clone, Debug)]
pub struct AppContext {
    pub league_id: u32,
    pub team_ids: Vec<u32>,
    pub gw: u32,
    pub fetch_sleep_ms: u64,
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
        let config: AppConfig = toml::from_str(config_file_content.as_str()).unwrap();
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


pub fn initialize_app_context(client: &Client, league_id: u32) -> AppContext {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let game = client.get_game().await.unwrap();
        let details = client.get_league_details(&league_id).await.unwrap();

        let team_ids = details.league_entries.iter().map(|x| x.entry_id).collect();

        let gw = game.current_event.expect("No game week found when initializing appContext, must be preseason!");

        let fetch_sleep_ms = 30_000 as u64;

        AppContext {
            league_id,
            team_ids,
            gw,
            fetch_sleep_ms,
        }
    })
}

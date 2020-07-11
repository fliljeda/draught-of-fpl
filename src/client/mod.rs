use reqwest::Client as ReqwestClient;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use crate::structs::{Game, Details, Live, TeamGw, TeamInfo, StaticInfo};
use serde::de;


const FPL_API_BASE: &str = "https://draft.premierleague.com/api/";
const LOCAL_API_BASE: &str = "/home/fl/db2/api";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ClientError {
    ReqwestError(String),
    InternalError(String),
    HttpError(String),
    LocalError(String),
    JsonError(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::InternalError(msg) => write!(f, "Internal client error: {}", msg),
            ClientError::ReqwestError(msg) => write!(f, "Reqwest lib error: {}", msg),
            ClientError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            ClientError::LocalError(msg) => write!(f, "Local error: {}", msg),
            ClientError::JsonError(msg) => write!(f, "Json error: {}", msg),
        }
    }
}

impl error::Error for ClientError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub struct Client {
    #[allow(dead_code)]
    http_client: ReqwestClient,
    local: bool,
}

fn deserialize_endpoint_struct<'a, T>(s: &'a str) -> Result<T, ClientError> where T: de::Deserialize<'a> {
    let o: T = match serde_json::from_str(&s){
        Ok(g) => g,
        Err(e) => {
            return Err(ClientError::ReqwestError(String::from(format!(
                "Error with processing request: {}",
                e
            ))))
        }
    };
    Ok(o)
}


// Basic client methods
impl Client {

    /* Creates and returns a new fpl client */
    pub fn new() -> Result<Client, ClientError> {
        let client_builder = ReqwestClient::builder().timeout(std::time::Duration::from_secs(10));

        let reqwest_client = match client_builder.build() {
            Ok(c) => c,
            Err(e) => {
                return Err(ClientError::ReqwestError(
                    format!("Could not create client with reason: {}", e).to_string()
                ))
            }
        };

        let client = Client {
            http_client: reqwest_client,
            local: false,
        };

        Ok(client)
    }

    pub fn new_local() -> Result<Client, ClientError> {
        let mut client = Client::new()?;
        client.set_local(true);
        Ok(client)
    }

    pub fn is_local(&self) -> bool {
        self.local
    }

    pub fn set_local(&mut self, local: bool) {
        self.local = local;
    }

    fn get_base_url(&self) -> &str {
        if self.is_local() {
            LOCAL_API_BASE
        } else {
            FPL_API_BASE
        }
    }

    async fn get(&self, path: &str) -> Result<String, ClientError> {
        if self.is_local() {
            self.fetch_file(path).await
        } else {
            self.fetch_web(path).await
        }
    }

    async fn fetch_file(&self, path: &str) -> Result<String, ClientError> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                return Err(ClientError::LocalError(format!(
                    "Error opening file with path {} {}",
                    path, e
                )))
            }
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(e) => {
                return Err(ClientError::LocalError(format!(
                    "Error reading file contents into string: {}",
                    e
                )))
            }
        }

        Ok(contents)
    }

    async fn fetch_web(&self, path: &str) -> Result<String, ClientError> {
        let resp =  match self.http_client.get(path).send().await {
            Ok(r) => r,
            Err(e) => {
                return Err(ClientError::ReqwestError(String::from(format!(
                    "Error with sending request: {}",
                    e
                ))))
            }
        };

        let body = match resp.text().await {
            Ok(b) => b,
            Err(e) => {
                return Err(ClientError::ReqwestError(String::from(format!(
                    "Error with processing request: {}",
                    e
                ))))
            }
        };

        Ok(body)
    }

}

// High level FPL fetch methods
impl Client {
    /* Fetches from /league/xxx/details endpoint */
    #[allow(dead_code)]
    pub async fn get_league_details(&self, league_code: &u32) -> Result<Details, ClientError> {
        let url = format!(
            "{api_base}/league/{league}/details",
            api_base = self.get_base_url(),
            league = league_code
        );
        let details = self.get(&url).await?;

        let details: Details = deserialize_endpoint_struct(&details)?;
        Ok(details)
    }

    #[allow(dead_code)]
    /* Fetches from /game endpoint */
    pub async fn get_game(&self) -> Result<Game, ClientError> {
        let url = format!("{api_base}/game", api_base = self.get_base_url());
        let game = self.get(&url).await?;

        let game: Game = deserialize_endpoint_struct(&game)?;
        Ok(game)
    }

    #[allow(dead_code)]
    /* Fetches from /entry/{team_code}/event/{gw} endpoint */
    pub async fn get_team_gw(&self, team: &u32, gw: &u32) -> Result<TeamGw, ClientError> {
        let url = format!(
            "{api_base}/entry/{team}/event/{gw}",
            api_base = self.get_base_url(),
            team = team,
            gw = gw
        );
        let team_gw = self.get(&url).await?;
        let team_gw = deserialize_endpoint_struct(&team_gw)?;
        Ok(team_gw)
    }

    #[allow(dead_code)]
    /* Fetches from /entry/{team_code}/public endpoint */
    pub async fn get_team_info(&self, team: &u32) -> Result<TeamInfo, ClientError> {
        let url = format!(
            "{api_base}/entry/{team}/public",
            api_base = self.get_base_url(),
            team = team
        );
        let team_info = self.get(&url).await?;
        let team_info = deserialize_endpoint_struct(&team_info)?;
        Ok(team_info)
    }

    #[allow(dead_code)]
    /* Fetches from event/{gw}/live endpoint */
    pub async fn get_gw_points_live(&self, gw: &u32) -> Result<Live, ClientError> {
        let url = format!(
            "{api_base}/event/{gw}/live",
            api_base = self.get_base_url(),
            gw = gw
        );
        let live = self.get(&url).await?;

        let live: Live = deserialize_endpoint_struct(&live)?;
        Ok(live)
    }

    #[allow(dead_code)]
    /* Fetches from /bootstrap-static endpoint */
    pub async fn get_static(&self) -> Result<StaticInfo, ClientError> {
        let url = format!(
            "{api_base}/bootstrap-static",
            api_base = self.get_base_url()
        );
        let static_info = self.get(&url).await?;
        let static_info = deserialize_endpoint_struct(&static_info)?;
        Ok(static_info)
    }
}

#[allow(dead_code)]
fn verify_error_code(code: reqwest::StatusCode) -> Result<(), ClientError> {
    match code.is_success() {
        true => Ok(()),
        false => Err(ClientError::HttpError(String::from(format!(
            "Received error code: {}",
            code
        )))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::join;

    #[tokio::test]
    async fn local_test_client() -> Result<(), ClientError>{
        let client = Client::new_local().unwrap();

        assert_eq!(client.local, true);
        assert_eq!(client.get_base_url(), LOCAL_API_BASE);

        let league_code: u32 = 305;
        let gw: u32 =  1;
        let team: u32 = 856;

        let game = client.get_game();
        let league_details = client.get_league_details(&league_code);
        let team_info = client.get_team_info(&team);
        let team_gw= client.get_team_gw(&team, &gw);
        let live= client.get_gw_points_live(&gw);
        let static_info = client.get_static();

        type EndpointsJoin = (
            Result<Game, ClientError>,
            Result<Details, ClientError>,
            Result<TeamInfo, ClientError>,
            Result<TeamGw, ClientError>,
            Result<Live, ClientError>,
            Result<StaticInfo, ClientError>,
        );

        let results: EndpointsJoin = join!(game, league_details, team_info, team_gw, live, static_info);

        results.0?;
        results.1?;
        results.2?;
        results.3?;
        results.4?;
        results.5?;

        Ok(())
    }

    #[tokio::test]
    #[ignore] //Expensive
    async fn web_test_client() -> Result<(), ClientError>{
        let client = Client::new().unwrap();
        assert_eq!(client.local, false);
        assert_eq!(client.get_base_url(), FPL_API_BASE);

        let game = client.get_game();
        let game2 = client.get_game();

        let (game,game2): (Result<Game, ClientError>,Result<Game, ClientError>) = join!(game, game2);

        game?;
        game2?;

        Ok(())
    }


    #[tokio::test]
    async fn local_endpoint_test_game() -> Result<(), ClientError> {
        let client = Client::new_local().unwrap();

        client.get_game().await?;

        Ok(())
    }

    #[tokio::test]
    async fn local_endpoint_test_league_details() -> Result<(), ClientError> {
        let client = Client::new_local().unwrap();

        let league_code: u32 = 305;
        client.get_league_details(&league_code).await?;

        Ok(())
    }

    #[tokio::test]
    async fn local_endpoint_test_live() -> Result<(), ClientError> {
        let client = Client::new_local().unwrap();

        let gw: u32 =  1;
        client.get_gw_points_live(&gw).await?;

        Ok(())
    }

    #[tokio::test]
    async fn local_endpoint_test_team_gw() -> Result<(), ClientError> {
        let client = Client::new_local().unwrap();

        let gw: u32 =  1;
        let team: u32 = 856;
        client.get_team_gw(&team, &gw).await?;

        Ok(())
    }

    #[tokio::test]
    async fn local_endpoint_test_team_info() -> Result<(), ClientError> {
        let client = Client::new_local().unwrap();

        let team: u32 = 856;
        client.get_team_info(&team).await?;

        Ok(())
    }

    #[tokio::test]
    async fn local_endpoint_test_static_info() -> Result<(), ClientError> {
        let client = Client::new_local().unwrap();

        client.get_static().await?;

        Ok(())
    }
}

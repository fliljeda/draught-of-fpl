use crate::structs::{Details, Game, Live, StaticInfo, TeamGw, TeamInfo};
use futures::future::join_all;
use reqwest::Client as ReqwestClient;
use serde::de;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

pub const FPL_API_BASE: &str = "https://draft.premierleague.com/api";
pub const DEFAULT_LOCAL_API_BASE: &str = "/fpl/api";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ClientError {
    Reqwest(String),
    Internal(String),
    Http(String),
    Local(String),
    Json(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::Internal(msg) => write!(f, "Internal client error: {}", msg),
            ClientError::Reqwest(msg) => write!(f, "Reqwest lib error: {}", msg),
            ClientError::Http(msg) => write!(f, "HTTP error: {}", msg),
            ClientError::Local(msg) => write!(f, "Local error: {}", msg),
            ClientError::Json(msg) => write!(f, "Json error: {}", msg),
        }
    }
}

impl error::Error for ClientError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

// TODO: Change to enum Local(), Web()
pub struct Client {
    #[allow(dead_code)]
    http_client: ReqwestClient,
    local: bool,
    local_url: Option<String>,
}

fn deserialize_endpoint_struct<'a, T>(s: &'a str) -> Result<T, ClientError>
where
    T: de::Deserialize<'a>,
{
    serde_json::from_str(s).map_err(|e| ClientError::Json(format!("Found Error: {}", e)))
}

// Basic client methods
impl Client {
    /* Creates and returns a new fpl client */
    pub fn new() -> Result<Client, ClientError> {
        let client_builder = ReqwestClient::builder().timeout(std::time::Duration::from_secs(10));

        let reqwest_client = match client_builder.build() {
            Ok(c) => c,
            Err(e) => {
                return Err(ClientError::Reqwest(
                    format!("Could not create client with reason: {}", e).to_string(),
                ))
            }
        };

        let client = Client {
            http_client: reqwest_client,
            local: false,
            local_url: None,
        };

        Ok(client)
    }

    pub fn new_local(local_url: Option<String>) -> Result<Client, ClientError> {
        let mut client = Client::new()?;
        client.set_local(true);
        client.local_url = local_url;
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
            if let Some(url) = &self.local_url {
                url.as_str()
            } else {
                DEFAULT_LOCAL_API_BASE
            }
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
                return Err(ClientError::Local(format!(
                    "Error opening file with path {} {}",
                    path, e
                )))
            }
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(e) => {
                return Err(ClientError::Local(format!(
                    "Error reading file contents into string: {}",
                    e
                )))
            }
        }

        Ok(contents)
    }

    async fn fetch_web(&self, path: &str) -> Result<String, ClientError> {
        tracing::trace!("Fetching {}", path);
        let resp = match self.http_client.get(path).send().await {
            Ok(r) => r,
            Err(e) => {
                return Err(ClientError::Reqwest(format!(
                    "Error with sending request: {}",
                    e
                )))
            }
        };

        verify_error_code(resp.status())?;

        let body = match resp.text().await {
            Ok(b) => b,
            Err(e) => {
                return Err(ClientError::Reqwest(format!(
                    "Error with processing request: {}",
                    e
                )))
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
    pub async fn get_multiple_teams_gw(
        &self,
        teams: &[u32],
        gw: &u32,
    ) -> HashMap<u32, Result<TeamGw, ClientError>> {
        let futures = teams
            .iter()
            .map(|team_id| self.get_team_gw(team_id, gw))
            .collect::<Vec<_>>();

        let team_gws: Vec<Result<TeamGw, ClientError>> = join_all(futures).await;

        let mut m: HashMap<u32, Result<TeamGw, ClientError>> = HashMap::new();

        // Assumes same ordering of teams and team_gws which futures::join_all guarantees
        team_gws.into_iter().enumerate().for_each(|(i, team_gw)| {
            m.insert(teams[i], team_gw);
        });
        m
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
    pub async fn get_multiple_teams_info(
        &self,
        teams: &[u32],
    ) -> HashMap<u32, Result<TeamInfo, ClientError>> {
        let futures = teams
            .iter()
            .map(|team_id| self.get_team_info(team_id))
            .collect::<Vec<_>>();

        let team_infos: Vec<Result<TeamInfo, ClientError>> = join_all(futures).await;

        let mut m: HashMap<u32, Result<TeamInfo, ClientError>> = HashMap::new();

        // Assumes same ordering of teams and team_gws which futures::join_all guarantees
        team_infos
            .into_iter()
            .enumerate()
            .for_each(|(i, team_info)| {
                m.insert(teams[i], team_info);
            });
        m
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

fn verify_error_code(code: reqwest::StatusCode) -> Result<(), ClientError> {
    match code.is_success() {
        true => Ok(()),
        false => Err(ClientError::Http(format!("Received error code: {}", code))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::join;

    #[test]
    fn success_status_is_ok() {
        assert!(verify_error_code(reqwest::StatusCode::OK).is_ok());
    }

    #[test]
    fn error_status_is_an_http_error() {
        let result = verify_error_code(reqwest::StatusCode::FORBIDDEN);

        assert!(matches!(result, Err(ClientError::Http(_))));
    }

    #[test]
    fn invalid_json_is_a_json_error() {
        let result = deserialize_endpoint_struct::<Game>("<html>Forbidden</html>");

        assert!(matches!(result, Err(ClientError::Json(_))));
    }

    #[tokio::test]
    #[ignore] //Expensive
    async fn web_test_client() -> Result<(), ClientError> {
        let client = Client::new().unwrap();
        assert!(!client.local);
        assert_eq!(client.get_base_url(), FPL_API_BASE);

        let game = client.get_game();
        let game2 = client.get_game();

        let (game, game2): (Result<Game, ClientError>, Result<Game, ClientError>) =
            join!(game, game2);

        game?;
        game2?;

        Ok(())
    }
}

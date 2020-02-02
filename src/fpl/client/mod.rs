use reqwest::Client as ReqwestClient;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

const FPL_API_BASE: &str = "https://draft.premierleague.com/api/";
const LOCAL_API_BASE: &str = "/home/fl/db/api";


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ClientError{
    ReqwestError(String),
    InternalError(String),
    HttpError(String),
    LocalError(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::InternalError(msg) => write!(f, "Internal client error: {}", msg),
            ClientError::ReqwestError(msg) => write!(f, "Reqwest lib error: {}", msg),
            ClientError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            ClientError::LocalError(msg) => write!(f, "Local error: {}", msg),
        }      
    }
}

impl error::Error for ClientError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}


pub struct Client {
    client: ReqwestClient,
    local: bool,
}

/* Creates and returns a new fpl client */
pub fn new() -> Result<Client, ClientError> {
    let client_builder = ReqwestClient::builder()
        .timeout(std::time::Duration::from_secs(10));

    let reqwest_client = match client_builder.build() {
        Ok(c) => c,
        Err(e) => return Err(ClientError::ReqwestError(String::from(format!("Could not create client with reason: {}", e)))),
    };

    let client = Client{
        client: reqwest_client,
        local: false,
    };

    Ok(client)
}

impl Client {
    fn get(&self, path: &str) -> Result<String, ClientError> {  
        if self.is_local() {
            self.fetch_local(path)
        } else {
            self.fetch_web(path)
        }
    }

    fn fetch_local(&self, path: &str) -> Result<String, ClientError> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(ClientError::LocalError(format!("Error opening file with path {} {}", path, e))),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(e) => return Err(ClientError::LocalError(format!("Error reading file contents into string: {}", e))),
        }

        Ok(contents)
    }

    fn fetch_web(&self, path: &str) -> Result<String, ClientError> {
        let mut resp = match self.client.get(path).send() {
            Ok(r) => r,
            Err(e) => return Err(ClientError::ReqwestError(String::from(format!("Error with sending request: {}", e)))),
        };

        verify_error_code(resp.status())?;

        
        let body = match resp.text() {
            Ok(b) => b,
            Err(e) => return Err(ClientError::ReqwestError(String::from(format!("Error with processing request: {}", e)))),
        };
                
        Ok(body)

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

    /* Fetches from /league/xxx/details endpoint */
    #[allow(dead_code)]
    pub fn get_league_details(&self, league_code: &str) -> Result<String, ClientError> {
        let url = format!("{api_base}league/{league}/details", api_base = self.get_base_url(), league = league_code);
        self.get(&url)
    }

    #[allow(dead_code)]
    /* Fetches from /game endpoint */
    pub fn get_game(&self) -> Result<String, ClientError> {
        let url = format!("{api_base}/game", api_base = self.get_base_url());
        self.get(&url)
    }

    #[allow(dead_code)]
    /* Fetches from /entry/{team_code}/event/{gw} endpoint */
    pub fn get_team_gw(&self, team: u32, gw: u32) -> Result<String, ClientError> {
        let url = format!("{api_base}/entry/{team}/event/{gw}", api_base = self.get_base_url(), team = team, gw = gw);
        self.get(&url)
    }

    #[allow(dead_code)]
    /* Fetches from /entry/{team_code}/public endpoint */
    pub fn get_team_info(&self, team: u32) -> Result<String, ClientError> {
        let url = format!("{api_base}/entry/{team}/public", api_base = self.get_base_url(), team = team);
        self.get(&url)
    }


    #[allow(dead_code)]
    /* Fetches from event/{gw}/live endpoint */
    pub fn get_gw_points_live(&self, gw: u32) -> Result<String, ClientError> {
        let url = format!("{api_base}/event/{gw}/live", api_base = self.get_base_url(), gw = gw);
        self.get(&url)
    }

    #[allow(dead_code)]
    /* Fetches from /bootstrap-static endpoint */
    pub fn get_static(&self) -> Result<String, ClientError> {
        let url = format!("{api_base}/bootstrap-static", api_base = self.get_base_url());
        self.get(&url)
    }
}

#[allow(dead_code)]
fn verify_error_code(code: reqwest::StatusCode) -> Result<(), ClientError> {
    match code.is_success() {
        true => Ok(()),
        false => Err(ClientError::HttpError(String::from(format!("Received error code: {}", code)))),
    }
}
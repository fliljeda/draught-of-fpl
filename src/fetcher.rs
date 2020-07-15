use std::sync::{Arc, RwLock};
use std::{thread, time};

use crate::client::{Client};
use crate::storage::FplEndpoints;
//use crate::structs::*;

// Continuously updates the endpoints
#[allow(dead_code)]
pub fn endpoint_cache_fetcher(endpoints_lock: Arc<RwLock<FplEndpoints>>, _client: &Client) {
    loop {
        thread::sleep(time::Duration::from_millis(10));

        //let new: FplEndpoints = fetch_new_endpoints(_client);
        match endpoints_lock.write() {
            Ok(mut t) => {
                println!("Grabbed the lock");
                //*t = new;
            }
            Err(e) => {
                println!("Could not grab write lock for table: {}", e);
            }
        };
    }
}

//async fn fetch_new_endpoints(client: &Client) -> FplEndpoints {
//    let league_code = 305;
//    let gw: u32 = 1;
//    let teams = Vec!(1,2,3,4,5);
//    let details = client.get_league_details(&league_code);
//    let game = client.get_game();
//    let (game, details): (Result<Game, ClientError>, Result<Details, ClientError>) = join!(details, game);
//
//
//    let live = client.get_gw_points_live(&gw);
//    let static_info = client.get_static();
//    let team_gw = client.get_team_gws(&teams, &gw);
//    let team_info = client.get_team_infos(&teams);
//
//    type EndpointsJoin = (
//        Result<Live, ClientError>,
//        Result<StaticInfo, ClientError>,
//        HashMap<u32, Result<TeamGw, ClientError>>,
//        HashMap<u32, Result<TeamInfo, ClientError>>,
//    );
//
//    let epj: EndpointsJoin = join!(live, static_info, team_gw, team_info)
//
//    FplEndpoints {
//        details: Some(details.unwrap()),
//        static_info: Some(static_info.unwrap()),
//        game: Some(game.unwrap()),
//        teams_info: Some(team_info.unwrap()),
//        teams_gw: Some(team_gw.unwrap()),
//        live: Some(live.unwrap())
//    }
//}

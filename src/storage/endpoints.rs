
use crate::structs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FplEndpoints {
    pub details: structs::Details,
    pub game: structs::Game,
    pub live: structs::Live,
    pub static_info: structs::StaticInfo,
    pub teams_gws: HashMap<u32, structs::TeamGw>,
    pub teams_infos: HashMap<u32, structs::TeamInfo>,
}

#[derive(Debug, Clone)]
pub struct FplEndpointsUpdate {
    pub details: Option<structs::Details>,
    pub game: Option<structs::Game>,
    pub live: Option<structs::Live>,
    pub static_info: Option<structs::StaticInfo>,
    pub teams_gws: HashMap<u32, Option<structs::TeamGw>>,
    pub teams_infos: HashMap<u32, Option<structs::TeamInfo>>,
}

impl FplEndpoints {

    pub fn initialize_from_update(update: FplEndpointsUpdate) -> FplEndpoints {
        let FplEndpointsUpdate {
            details,
            game,
            live,
            static_info,
            teams_gws,
            teams_infos
        } = update;

        let details = details.unwrap();
        let game = game.unwrap();
        let live = live.unwrap();
        let static_info = static_info.unwrap();

        let mut new_teams_gws: HashMap<u32, structs::TeamGw> = HashMap::new();
        teams_gws.into_iter().for_each( |(key,val)| {
            new_teams_gws.insert(key, val.unwrap());
        });


        let mut new_teams_infos: HashMap<u32, structs::TeamInfo> = HashMap::new();
        teams_infos.into_iter().for_each( |(key,val)| {
            new_teams_infos.insert(key, val.unwrap());
        });

        FplEndpoints {
            details,
            game,
            live,
            static_info,
            teams_gws: new_teams_gws,
            teams_infos: new_teams_infos,
        }
    }

    pub fn update(&mut self, other: FplEndpointsUpdate){
        let FplEndpointsUpdate {
            details,
            game,
            live,
            static_info,
            teams_gws,
            teams_infos
        } = other;


        if let Some(game) = game {
            self.game = game
        }

        if let Some(live) = live {
            self.live = live;
        }

        if let Some(details) = details {
            self.details = details;
        }

        if let Some(static_info) = static_info {
            self.static_info = static_info;
        }

        for (team_id, team_gw) in teams_gws.into_iter() {
            if let Some(team_gw) = team_gw {
                self.teams_gws.insert(team_id, team_gw);
            }
        }

        for (team_id, team_info) in teams_infos.into_iter() {
            if let Some(team_info) = team_info {
                self.teams_infos.insert(team_id, team_info);
            }
        }
    }
}


use crate::structs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FplEndpoints {
    pub details: Option<structs::Details>,
    pub game: Option<structs::Game>,
    pub live: Option<structs::Live>,
    pub static_info: Option<structs::StaticInfo>,
    pub teams_gws: HashMap<u32, Option<structs::TeamGw>>,
    pub teams_infos: HashMap<u32, Option<structs::TeamInfo>>,
}

impl FplEndpoints {
    pub fn create_blank() -> FplEndpoints {
        FplEndpoints {
            details: None,
            game: None,
            live: None,
            static_info: None,
            teams_gws: HashMap::new(),
            teams_infos: HashMap::new(),
        }
    }

    pub fn update(&mut self, other: FplEndpoints){

        let FplEndpoints {
            details,
            game,
            live,
            static_info,
            teams_gws,
            teams_infos
        } = other;

        if game.is_some() {
            self.game = game;
        }

        if live.is_none() {
            self.live = live;
        }

        if details.is_some() {
            self.details = details;
        }

        if static_info.is_some() {
            self.static_info = static_info;
        }

        for (team_id, team_gw) in teams_gws.into_iter() {
            if team_gw.is_some() {
                self.teams_gws.insert(team_id, team_gw);
            }
        }

        for (team_id, team_info) in teams_infos.into_iter() {
            if team_info.is_some() {
                self.teams_infos.insert(team_id, team_info);
            }
        }
    }
}

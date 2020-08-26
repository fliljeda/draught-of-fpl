// The property computer
use crate::storage::FplEndpoints;
use crate::structs::staticinfo::Element;

// Gets the display name for a FPL player
pub fn get_player_display_name(endpoints: &FplEndpoints, player_id: u32) -> String {
    return match get_player(endpoints, player_id) {
        Some(p) => String::from(p.web_name.as_ref().unwrap()),
        None => format!("<Unknown player: {}>", player_id)
    }
}

pub fn get_player_full_name(endpoints: &FplEndpoints, player_id: u32) -> String {
    return match get_player(endpoints, player_id) {
        Some(p) => format!("{} {}", p.first_name.as_ref().unwrap(), p.second_name.as_ref().unwrap()),
        None => format!("<Unknown player: {}>", player_id)
    }
}

fn get_player(endpoints: &FplEndpoints, player_id: u32) -> Option<&Element> {
    if let Some(static_info) = &endpoints.static_info {
        let i = (player_id - 1) as usize;
        return static_info.elements.get(i);
    } else {
        None
    }
}
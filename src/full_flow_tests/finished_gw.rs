// GW 8 after every match has finished and all bonus points are official.

use std::sync::LazyLock;

use serde_json::{json, Value};

use super::*;

// Built once by the first test that uses it, then shared by the others
static FINISHED_GW: LazyLock<Value> = LazyLock::new(|| get_table("2026-10-31-gw8-finished-gw"));

// Team points are FPL's own: `standings` in the league details response.
// Player values are the program's output for this snapshot.

#[test]
fn teams_are_ordered_by_total_points() {
    assert_eq!(FINISHED_GW["entries"][0]["team_name"], TEAM_2);
    assert_eq!(FINISHED_GW["entries"][1]["team_name"], TEAM_5);
    assert_eq!(FINISHED_GW["entries"][2]["team_name"], TEAM_6);
    assert_eq!(FINISHED_GW["entries"][3]["team_name"], TEAM_3);
    assert_eq!(FINISHED_GW["entries"][4]["team_name"], TEAM_1);
    assert_eq!(FINISHED_GW["entries"][5]["team_name"], TEAM_4);
}

#[test]
fn team_total_points() {
    expect_team(TEAM_1, "total_points", 339);
    expect_team(TEAM_2, "total_points", 376);
    expect_team(TEAM_3, "total_points", 341);
    expect_team(TEAM_4, "total_points", 319);
    expect_team(TEAM_5, "total_points", 359);
    expect_team(TEAM_6, "total_points", 357);
}

#[test]
fn team_gameweek_points() {
    expect_team(TEAM_1, "gw_points", 50);
    expect_team(TEAM_2, "gw_points", 50);
    expect_team(TEAM_3, "gw_points", 52);
    expect_team(TEAM_4, "gw_points", 32);
    expect_team(TEAM_5, "gw_points", 24);
    expect_team(TEAM_6, "gw_points", 50);
}

#[test]
fn team_total_projected_points() {
    expect_team(TEAM_1, "total_projected_points", 339);
    expect_team(TEAM_2, "total_projected_points", 376);
    expect_team(TEAM_3, "total_projected_points", 341);
    expect_team(TEAM_4, "total_projected_points", 319);
    expect_team(TEAM_5, "total_projected_points", 359);
    expect_team(TEAM_6, "total_projected_points", 357);
}

#[test]
fn team_gameweek_projected_points() {
    expect_team(TEAM_1, "gw_projected_points", 50);
    expect_team(TEAM_2, "gw_projected_points", 50);
    expect_team(TEAM_3, "gw_projected_points", 52);
    expect_team(TEAM_4, "gw_projected_points", 32);
    expect_team(TEAM_5, "gw_projected_points", 24);
    expect_team(TEAM_6, "gw_projected_points", 50);
}

#[test]
fn team_projected_points_explanation_is_empty() {
    expect_team(TEAM_1, "projected_points_explanation", json!([]));
    expect_team(TEAM_2, "projected_points_explanation", json!([]));
    expect_team(TEAM_3, "projected_points_explanation", json!([]));
    expect_team(TEAM_4, "projected_points_explanation", json!([]));
    expect_team(TEAM_5, "projected_points_explanation", json!([]));
    expect_team(TEAM_6, "projected_points_explanation", json!([]));
}

#[test]
fn player_points() {
    // Team 01
    expect_player("Donnarumma", "points", 1);
    expect_player("Davis", "points", 2);
    expect_player("Guéhi", "points", 0);
    expect_player("Calafiori", "points", 1);
    expect_player("Virgil", "points", 9);
    expect_player("Wirtz", "points", 6);
    expect_player("Rayan", "points", 10);
    expect_player("Barcola", "points", 6);
    expect_player("Isak", "points", 9);
    expect_player("Solanke", "points", 1);
    expect_player("Thiago", "points", 5);
    expect_player("Kelleher", "points", 2);
    expect_player("Muñoz", "points", 7);
    expect_player("Hinshelwood", "points", 1);
    expect_player("Groß", "points", 0);

    // Team 02
    expect_player("A.Becker", "points", 6);
    expect_player("Jacquet", "points", 8);
    expect_player("White", "points", 10);
    expect_player("Gvardiol", "points", 1);
    expect_player("Bogle", "points", 5);
    expect_player("Thomas", "points", 1);
    expect_player("Mbeumo", "points", 2);
    expect_player("Bruno G.", "points", 1);
    expect_player("Barnes", "points", 8);
    expect_player("Thomas-Asante", "points", 2);
    expect_player("Evanilson", "points", 6);
    expect_player("Horníček", "points", 8);
    expect_player("B.Fernandes", "points", 0);
    expect_player("McBurnie", "points", 8);
    expect_player("Tzolis", "points", 0);

    // Team 03
    expect_player("Trafford", "points", 10);
    expect_player("Konsa", "points", 14);
    expect_player("Van Hecke", "points", 7);
    expect_player("Vuskovic", "points", 1);
    expect_player("Ajer", "points", 2);
    expect_player("Enzo", "points", 2);
    expect_player("Palmer", "points", -1);
    expect_player("Cherki", "points", 0);
    expect_player("Schade", "points", 2);
    expect_player("Delap", "points", 2);
    expect_player("N.Jackson", "points", 13);
    expect_player("Kinsky", "points", 9);
    expect_player("Brobbey", "points", 2);
    expect_player("Kroupi.Jr", "points", 1);
    expect_player("Araujo", "points", 10);

    // Team 04
    expect_player("Tzolakis", "points", 3);
    expect_player("Tarkowski", "points", 2);
    expect_player("Mykolenko", "points", 0);
    expect_player("Hall", "points", 5);
    expect_player("Neto", "points", 3);
    expect_player("Rogers", "points", 1);
    expect_player("Ødegaard", "points", 3);
    expect_player("Tavernier", "points", 10);
    expect_player("Xhaka", "points", 3);
    expect_player("Fernandez-Pardo", "points", 0);
    expect_player("Kostoulas", "points", 2);
    expect_player("Suzuki", "points", 4);
    expect_player("João Pedro", "points", 0);
    expect_player("Maitland-Niles", "points", 0);
    expect_player("Branthwaite", "points", 0);

    // Team 05
    expect_player("Pickford", "points", 1);
    expect_player("Rúben", "points", 1);
    expect_player("De Cuyper", "points", 1);
    expect_player("Mukiele", "points", 1);
    expect_player("Gibbs-White", "points", 2);
    expect_player("Cunha", "points", 1);
    expect_player("Foden", "points", 1);
    expect_player("Ndiaye", "points", 5);
    expect_player("Barry", "points", 2);
    expect_player("Haaland", "points", 8);
    expect_player("Emersonn", "points", 1);
    expect_player("Henderson", "points", 0);
    expect_player("Rashford", "points", 0);
    expect_player("Senesi", "points", 0);
    expect_player("Matheus N.", "points", 0);

    // Team 06
    expect_player("Raya", "points", 7);
    expect_player("Gabriel", "points", 17);
    expect_player("Castagne", "points", 7);
    expect_player("Ajayi", "points", 4);
    expect_player("Milenković", "points", 2);
    expect_player("Rice", "points", 3);
    expect_player("Gomez", "points", 2);
    expect_player("Doku", "points", 2);
    expect_player("Gakpo", "points", 3);
    expect_player("Havertz", "points", 1);
    expect_player("Calvert-Lewin", "points", 2);
    expect_player("Verbruggen", "points", 2);
    expect_player("Isidor", "points", 0);
    expect_player("J.Timber", "points", 0);
    expect_player("Semenyo", "points", 0);
}

#[test]
fn player_projected_points() {
    // Team 01
    expect_player("Donnarumma", "projected_points", 1);
    expect_player("Davis", "projected_points", 2);
    expect_player("Guéhi", "projected_points", 0);
    expect_player("Calafiori", "projected_points", 1);
    expect_player("Virgil", "projected_points", 9);
    expect_player("Wirtz", "projected_points", 6);
    expect_player("Rayan", "projected_points", 10);
    expect_player("Barcola", "projected_points", 6);
    expect_player("Isak", "projected_points", 9);
    expect_player("Solanke", "projected_points", 1);
    expect_player("Thiago", "projected_points", 5);
    expect_player("Kelleher", "projected_points", 2);
    expect_player("Muñoz", "projected_points", 7);
    expect_player("Hinshelwood", "projected_points", 1);
    expect_player("Groß", "projected_points", 0);

    // Team 02
    expect_player("A.Becker", "projected_points", 6);
    expect_player("Jacquet", "projected_points", 8);
    expect_player("White", "projected_points", 10);
    expect_player("Gvardiol", "projected_points", 1);
    expect_player("Bogle", "projected_points", 5);
    expect_player("Thomas", "projected_points", 1);
    expect_player("Mbeumo", "projected_points", 2);
    expect_player("Bruno G.", "projected_points", 1);
    expect_player("Barnes", "projected_points", 8);
    expect_player("Thomas-Asante", "projected_points", 2);
    expect_player("Evanilson", "projected_points", 6);
    expect_player("Horníček", "projected_points", 8);
    expect_player("B.Fernandes", "projected_points", 0);
    expect_player("McBurnie", "projected_points", 8);
    expect_player("Tzolis", "projected_points", 0);

    // Team 03
    expect_player("Trafford", "projected_points", 10);
    expect_player("Konsa", "projected_points", 14);
    expect_player("Van Hecke", "projected_points", 7);
    expect_player("Vuskovic", "projected_points", 1);
    expect_player("Ajer", "projected_points", 2);
    expect_player("Enzo", "projected_points", 2);
    expect_player("Palmer", "projected_points", -1);
    expect_player("Cherki", "projected_points", 0);
    expect_player("Schade", "projected_points", 2);
    expect_player("Delap", "projected_points", 2);
    expect_player("N.Jackson", "projected_points", 13);
    expect_player("Kinsky", "projected_points", 9);
    expect_player("Brobbey", "projected_points", 2);
    expect_player("Kroupi.Jr", "projected_points", 1);
    expect_player("Araujo", "projected_points", 10);

    // Team 04
    expect_player("Tzolakis", "projected_points", 3);
    expect_player("Tarkowski", "projected_points", 2);
    expect_player("Mykolenko", "projected_points", 0);
    expect_player("Hall", "projected_points", 5);
    expect_player("Neto", "projected_points", 3);
    expect_player("Rogers", "projected_points", 1);
    expect_player("Ødegaard", "projected_points", 3);
    expect_player("Tavernier", "projected_points", 10);
    expect_player("Xhaka", "projected_points", 3);
    expect_player("Fernandez-Pardo", "projected_points", 0);
    expect_player("Kostoulas", "projected_points", 2);
    expect_player("Suzuki", "projected_points", 4);
    expect_player("João Pedro", "projected_points", 0);
    expect_player("Maitland-Niles", "projected_points", 0);
    expect_player("Branthwaite", "projected_points", 0);

    // Team 05
    expect_player("Pickford", "projected_points", 1);
    expect_player("Rúben", "projected_points", 1);
    expect_player("De Cuyper", "projected_points", 1);
    expect_player("Mukiele", "projected_points", 1);
    expect_player("Gibbs-White", "projected_points", 2);
    expect_player("Cunha", "projected_points", 1);
    expect_player("Foden", "projected_points", 1);
    expect_player("Ndiaye", "projected_points", 5);
    expect_player("Barry", "projected_points", 2);
    expect_player("Haaland", "projected_points", 8);
    expect_player("Emersonn", "projected_points", 1);
    expect_player("Henderson", "projected_points", 0);
    expect_player("Rashford", "projected_points", 0);
    expect_player("Senesi", "projected_points", 0);
    expect_player("Matheus N.", "projected_points", 0);

    // Team 06
    expect_player("Raya", "projected_points", 7);
    expect_player("Gabriel", "projected_points", 17);
    expect_player("Castagne", "projected_points", 7);
    expect_player("Ajayi", "projected_points", 4);
    expect_player("Milenković", "projected_points", 2);
    expect_player("Rice", "projected_points", 3);
    expect_player("Gomez", "projected_points", 2);
    expect_player("Doku", "projected_points", 2);
    expect_player("Gakpo", "projected_points", 3);
    expect_player("Havertz", "projected_points", 1);
    expect_player("Calvert-Lewin", "projected_points", 2);
    expect_player("Verbruggen", "projected_points", 2);
    expect_player("Isidor", "projected_points", 0);
    expect_player("J.Timber", "projected_points", 0);
    expect_player("Semenyo", "projected_points", 0);
}

#[test]
fn player_bps() {
    // Team 01
    expect_player("Donnarumma", "bps", -4);
    expect_player("Davis", "bps", 20);
    expect_player("Guéhi", "bps", -1);
    expect_player("Calafiori", "bps", 4);
    expect_player("Virgil", "bps", 27);
    expect_player("Wirtz", "bps", 23);
    expect_player("Rayan", "bps", 29);
    expect_player("Barcola", "bps", 21);
    expect_player("Isak", "bps", 33);
    expect_player("Solanke", "bps", 7);
    expect_player("Thiago", "bps", 19);
    expect_player("Kelleher", "bps", 6);
    expect_player("Muñoz", "bps", 28);
    expect_player("Hinshelwood", "bps", 5);
    expect_player("Groß", "bps", 15);

    // Team 02
    expect_player("A.Becker", "bps", 23);
    expect_player("Jacquet", "bps", 19);
    expect_player("White", "bps", 38);
    expect_player("Gvardiol", "bps", 13);
    expect_player("Bogle", "bps", 21);
    expect_player("Thomas", "bps", 7);
    expect_player("Mbeumo", "bps", 14);
    expect_player("Bruno G.", "bps", 6);
    expect_player("Barnes", "bps", 27);
    expect_player("Thomas-Asante", "bps", 10);
    expect_player("Evanilson", "bps", 33);
    expect_player("Horníček", "bps", 33);
    expect_player("B.Fernandes", "bps", 0);
    expect_player("McBurnie", "bps", 23);
    expect_player("Tzolis", "bps", 0);

    // Team 03
    expect_player("Trafford", "bps", 33);
    expect_player("Konsa", "bps", 43);
    expect_player("Van Hecke", "bps", 28);
    expect_player("Vuskovic", "bps", 6);
    expect_player("Ajer", "bps", 10);
    expect_player("Enzo", "bps", 18);
    expect_player("Palmer", "bps", 1);
    expect_player("Cherki", "bps", 0);
    expect_player("Schade", "bps", 7);
    expect_player("Delap", "bps", 1);
    expect_player("N.Jackson", "bps", 66);
    expect_player("Kinsky", "bps", 33);
    expect_player("Brobbey", "bps", 6);
    expect_player("Kroupi.Jr", "bps", 4);
    expect_player("Araujo", "bps", 29);

    // Team 04
    expect_player("Tzolakis", "bps", 22);
    expect_player("Tarkowski", "bps", 8);
    expect_player("Mykolenko", "bps", -4);
    expect_player("Hall", "bps", 25);
    expect_player("Neto", "bps", 10);
    expect_player("Rogers", "bps", 7);
    expect_player("Ødegaard", "bps", 15);
    expect_player("Tavernier", "bps", 40);
    expect_player("Xhaka", "bps", 17);
    expect_player("Fernandez-Pardo", "bps", 0);
    expect_player("Kostoulas", "bps", 15);
    expect_player("Suzuki", "bps", 26);
    expect_player("João Pedro", "bps", 0);
    expect_player("Maitland-Niles", "bps", 0);
    expect_player("Branthwaite", "bps", 0);

    // Team 05
    expect_player("Pickford", "bps", 7);
    expect_player("Rúben", "bps", 3);
    expect_player("De Cuyper", "bps", 6);
    expect_player("Mukiele", "bps", 6);
    expect_player("Gibbs-White", "bps", 15);
    expect_player("Cunha", "bps", 15);
    expect_player("Foden", "bps", 6);
    expect_player("Ndiaye", "bps", 25);
    expect_player("Barry", "bps", 11);
    expect_player("Haaland", "bps", 35);
    expect_player("Emersonn", "bps", 5);
    expect_player("Henderson", "bps", 0);
    expect_player("Rashford", "bps", 0);
    expect_player("Senesi", "bps", 0);
    expect_player("Matheus N.", "bps", 0);

    // Team 06
    expect_player("Raya", "bps", 29);
    expect_player("Gabriel", "bps", 44);
    expect_player("Castagne", "bps", 32);
    expect_player("Ajayi", "bps", 15);
    expect_player("Milenković", "bps", 7);
    expect_player("Rice", "bps", 23);
    expect_player("Gomez", "bps", 12);
    expect_player("Doku", "bps", 27);
    expect_player("Gakpo", "bps", 11);
    expect_player("Havertz", "bps", 6);
    expect_player("Calvert-Lewin", "bps", 5);
    expect_player("Verbruggen", "bps", 11);
    expect_player("Isidor", "bps", 0);
    expect_player("J.Timber", "bps", 0);
    expect_player("Semenyo", "bps", 0);
}

#[test]
fn player_on_field() {
    // Team 01
    expect_player("Donnarumma", "on_field", true);
    expect_player("Davis", "on_field", true);
    expect_player("Guéhi", "on_field", true);
    expect_player("Calafiori", "on_field", true);
    expect_player("Virgil", "on_field", true);
    expect_player("Wirtz", "on_field", true);
    expect_player("Rayan", "on_field", true);
    expect_player("Barcola", "on_field", true);
    expect_player("Isak", "on_field", true);
    expect_player("Solanke", "on_field", true);
    expect_player("Thiago", "on_field", true);
    expect_player("Kelleher", "on_field", false);
    expect_player("Muñoz", "on_field", false);
    expect_player("Hinshelwood", "on_field", false);
    expect_player("Groß", "on_field", false);

    // Team 02
    expect_player("A.Becker", "on_field", true);
    expect_player("Jacquet", "on_field", true);
    expect_player("White", "on_field", true);
    expect_player("Gvardiol", "on_field", true);
    expect_player("Bogle", "on_field", true);
    expect_player("Thomas", "on_field", true);
    expect_player("Mbeumo", "on_field", true);
    expect_player("Bruno G.", "on_field", true);
    expect_player("Barnes", "on_field", true);
    expect_player("Thomas-Asante", "on_field", true);
    expect_player("Evanilson", "on_field", true);
    expect_player("Horníček", "on_field", false);
    expect_player("B.Fernandes", "on_field", false);
    expect_player("McBurnie", "on_field", false);
    expect_player("Tzolis", "on_field", false);

    // Team 03
    expect_player("Trafford", "on_field", true);
    expect_player("Konsa", "on_field", true);
    expect_player("Van Hecke", "on_field", true);
    expect_player("Vuskovic", "on_field", true);
    expect_player("Ajer", "on_field", true);
    expect_player("Enzo", "on_field", true);
    expect_player("Palmer", "on_field", true);
    expect_player("Cherki", "on_field", true);
    expect_player("Schade", "on_field", true);
    expect_player("Delap", "on_field", true);
    expect_player("N.Jackson", "on_field", true);
    expect_player("Kinsky", "on_field", false);
    expect_player("Brobbey", "on_field", false);
    expect_player("Kroupi.Jr", "on_field", false);
    expect_player("Araujo", "on_field", false);

    // Team 04
    expect_player("Tzolakis", "on_field", true);
    expect_player("Tarkowski", "on_field", true);
    expect_player("Mykolenko", "on_field", true);
    expect_player("Hall", "on_field", true);
    expect_player("Neto", "on_field", true);
    expect_player("Rogers", "on_field", true);
    expect_player("Ødegaard", "on_field", true);
    expect_player("Tavernier", "on_field", true);
    expect_player("Xhaka", "on_field", true);
    expect_player("Fernandez-Pardo", "on_field", true);
    expect_player("Kostoulas", "on_field", true);
    expect_player("Suzuki", "on_field", false);
    expect_player("João Pedro", "on_field", false);
    expect_player("Maitland-Niles", "on_field", false);
    expect_player("Branthwaite", "on_field", false);

    // Team 05
    expect_player("Pickford", "on_field", true);
    expect_player("Rúben", "on_field", true);
    expect_player("De Cuyper", "on_field", true);
    expect_player("Mukiele", "on_field", true);
    expect_player("Gibbs-White", "on_field", true);
    expect_player("Cunha", "on_field", true);
    expect_player("Foden", "on_field", true);
    expect_player("Ndiaye", "on_field", true);
    expect_player("Barry", "on_field", true);
    expect_player("Haaland", "on_field", true);
    expect_player("Emersonn", "on_field", true);
    expect_player("Henderson", "on_field", false);
    expect_player("Rashford", "on_field", false);
    expect_player("Senesi", "on_field", false);
    expect_player("Matheus N.", "on_field", false);

    // Team 06
    expect_player("Raya", "on_field", true);
    expect_player("Gabriel", "on_field", true);
    expect_player("Castagne", "on_field", true);
    expect_player("Ajayi", "on_field", true);
    expect_player("Milenković", "on_field", true);
    expect_player("Rice", "on_field", true);
    expect_player("Gomez", "on_field", true);
    expect_player("Doku", "on_field", true);
    expect_player("Gakpo", "on_field", true);
    expect_player("Havertz", "on_field", true);
    expect_player("Calvert-Lewin", "on_field", true);
    expect_player("Verbruggen", "on_field", false);
    expect_player("Isidor", "on_field", false);
    expect_player("J.Timber", "on_field", false);
    expect_player("Semenyo", "on_field", false);
}

#[test]
fn player_has_played() {
    // Team 01
    expect_player("Donnarumma", "has_played", true);
    expect_player("Davis", "has_played", true);
    expect_player("Guéhi", "has_played", true);
    expect_player("Calafiori", "has_played", true);
    expect_player("Virgil", "has_played", true);
    expect_player("Wirtz", "has_played", true);
    expect_player("Rayan", "has_played", true);
    expect_player("Barcola", "has_played", true);
    expect_player("Isak", "has_played", true);
    expect_player("Solanke", "has_played", true);
    expect_player("Thiago", "has_played", true);
    expect_player("Kelleher", "has_played", true);
    expect_player("Muñoz", "has_played", true);
    expect_player("Hinshelwood", "has_played", true);
    expect_player("Groß", "has_played", true);

    // Team 02
    expect_player("A.Becker", "has_played", true);
    expect_player("Jacquet", "has_played", true);
    expect_player("White", "has_played", true);
    expect_player("Gvardiol", "has_played", true);
    expect_player("Bogle", "has_played", true);
    expect_player("Thomas", "has_played", true);
    expect_player("Mbeumo", "has_played", true);
    expect_player("Bruno G.", "has_played", true);
    expect_player("Barnes", "has_played", true);
    expect_player("Thomas-Asante", "has_played", true);
    expect_player("Evanilson", "has_played", true);
    expect_player("Horníček", "has_played", true);
    expect_player("B.Fernandes", "has_played", false);
    expect_player("McBurnie", "has_played", true);
    expect_player("Tzolis", "has_played", false);

    // Team 03
    expect_player("Trafford", "has_played", true);
    expect_player("Konsa", "has_played", true);
    expect_player("Van Hecke", "has_played", true);
    expect_player("Vuskovic", "has_played", true);
    expect_player("Ajer", "has_played", true);
    expect_player("Enzo", "has_played", true);
    expect_player("Palmer", "has_played", true);
    expect_player("Cherki", "has_played", true);
    expect_player("Schade", "has_played", true);
    expect_player("Delap", "has_played", true);
    expect_player("N.Jackson", "has_played", true);
    expect_player("Kinsky", "has_played", true);
    expect_player("Brobbey", "has_played", true);
    expect_player("Kroupi.Jr", "has_played", true);
    expect_player("Araujo", "has_played", true);

    // Team 04
    expect_player("Tzolakis", "has_played", true);
    expect_player("Tarkowski", "has_played", true);
    expect_player("Mykolenko", "has_played", true);
    expect_player("Hall", "has_played", true);
    expect_player("Neto", "has_played", true);
    expect_player("Rogers", "has_played", true);
    expect_player("Ødegaard", "has_played", true);
    expect_player("Tavernier", "has_played", true);
    expect_player("Xhaka", "has_played", true);
    expect_player("Fernandez-Pardo", "has_played", false);
    expect_player("Kostoulas", "has_played", true);
    expect_player("Suzuki", "has_played", true);
    expect_player("João Pedro", "has_played", false);
    expect_player("Maitland-Niles", "has_played", false);
    expect_player("Branthwaite", "has_played", false);

    // Team 05
    expect_player("Pickford", "has_played", true);
    expect_player("Rúben", "has_played", true);
    expect_player("De Cuyper", "has_played", true);
    expect_player("Mukiele", "has_played", true);
    expect_player("Gibbs-White", "has_played", true);
    expect_player("Cunha", "has_played", true);
    expect_player("Foden", "has_played", true);
    expect_player("Ndiaye", "has_played", true);
    expect_player("Barry", "has_played", true);
    expect_player("Haaland", "has_played", true);
    expect_player("Emersonn", "has_played", true);
    expect_player("Henderson", "has_played", false);
    expect_player("Rashford", "has_played", false);
    expect_player("Senesi", "has_played", false);
    expect_player("Matheus N.", "has_played", false);

    // Team 06
    expect_player("Raya", "has_played", true);
    expect_player("Gabriel", "has_played", true);
    expect_player("Castagne", "has_played", true);
    expect_player("Ajayi", "has_played", true);
    expect_player("Milenković", "has_played", true);
    expect_player("Rice", "has_played", true);
    expect_player("Gomez", "has_played", true);
    expect_player("Doku", "has_played", true);
    expect_player("Gakpo", "has_played", true);
    expect_player("Havertz", "has_played", true);
    expect_player("Calvert-Lewin", "has_played", true);
    expect_player("Verbruggen", "has_played", true);
    expect_player("Isidor", "has_played", false);
    expect_player("J.Timber", "has_played", false);
    expect_player("Semenyo", "has_played", false);
}

#[test]
fn player_play_status() {
    // Team 01
    expect_play_status("Donnarumma", "playing");
    expect_play_status("Davis", "playing");
    expect_play_status("Guéhi", "playing");
    expect_play_status("Calafiori", "playing");
    expect_play_status("Virgil", "playing");
    expect_play_status("Wirtz", "playing");
    expect_play_status("Rayan", "playing");
    expect_play_status("Barcola", "playing");
    expect_play_status("Isak", "playing");
    expect_play_status("Solanke", "playing");
    expect_play_status("Thiago", "playing");
    expect_play_status("Kelleher", "benched");
    expect_play_status("Muñoz", "benched");
    expect_play_status("Hinshelwood", "benched");
    expect_play_status("Groß", "benched");

    // Team 02
    expect_play_status("A.Becker", "playing");
    expect_play_status("Jacquet", "playing");
    expect_play_status("White", "playing");
    expect_play_status("Gvardiol", "playing");
    expect_play_status("Bogle", "playing");
    expect_play_status("Thomas", "playing");
    expect_play_status("Mbeumo", "playing");
    expect_play_status("Bruno G.", "playing");
    expect_play_status("Barnes", "playing");
    expect_play_status("Thomas-Asante", "playing");
    expect_play_status("Evanilson", "playing");
    expect_play_status("Horníček", "benched");
    expect_play_status("B.Fernandes", "benched");
    expect_play_status("McBurnie", "benched");
    expect_play_status("Tzolis", "benched");

    // Team 03
    expect_play_status("Trafford", "playing");
    expect_play_status("Konsa", "playing");
    expect_play_status("Van Hecke", "playing");
    expect_play_status("Vuskovic", "playing");
    expect_play_status("Ajer", "playing");
    expect_play_status("Enzo", "playing");
    expect_play_status("Palmer", "playing");
    expect_play_status("Cherki", "playing");
    expect_play_status("Schade", "playing");
    expect_play_status("Delap", "playing");
    expect_play_status("N.Jackson", "playing");
    expect_play_status("Kinsky", "benched");
    expect_play_status("Brobbey", "benched");
    expect_play_status("Kroupi.Jr", "benched");
    expect_play_status("Araujo", "benched");

    // Team 04
    expect_play_status("Tzolakis", "playing");
    expect_play_status("Tarkowski", "playing");
    expect_play_status("Mykolenko", "playing");
    expect_play_status("Hall", "playing");
    expect_play_status("Neto", "playing");
    expect_play_status("Rogers", "playing");
    expect_play_status("Ødegaard", "playing");
    expect_play_status("Tavernier", "playing");
    expect_play_status("Xhaka", "playing");
    expect_play_status("Fernandez-Pardo", "playing");
    expect_play_status("Kostoulas", "playing");
    expect_play_status("Suzuki", "benched");
    expect_play_status("João Pedro", "benched");
    expect_play_status("Maitland-Niles", "benched");
    expect_play_status("Branthwaite", "benched");

    // Team 05
    expect_play_status("Pickford", "playing");
    expect_play_status("Rúben", "playing");
    expect_play_status("De Cuyper", "playing");
    expect_play_status("Mukiele", "playing");
    expect_play_status("Gibbs-White", "playing");
    expect_play_status("Cunha", "playing");
    expect_play_status("Foden", "playing");
    expect_play_status("Ndiaye", "playing");
    expect_play_status("Barry", "playing");
    expect_play_status("Haaland", "playing");
    expect_play_status("Emersonn", "playing");
    expect_play_status("Henderson", "benched");
    expect_play_status("Rashford", "benched");
    expect_play_status("Senesi", "benched");
    expect_play_status("Matheus N.", "benched");

    // Team 06
    expect_play_status("Raya", "playing");
    expect_play_status("Gabriel", "playing");
    expect_play_status("Castagne", "playing");
    expect_play_status("Ajayi", "playing");
    expect_play_status("Milenković", "playing");
    expect_play_status("Rice", "playing");
    expect_play_status("Gomez", "playing");
    expect_play_status("Doku", "playing");
    expect_play_status("Gakpo", "playing");
    expect_play_status("Havertz", "playing");
    expect_play_status("Calvert-Lewin", "playing");
    expect_play_status("Verbruggen", "benched");
    expect_play_status("Isidor", "benched");
    expect_play_status("J.Timber", "benched");
    expect_play_status("Semenyo", "benched");
}

// Fernandez-Pardo did not play, but the only bench player who played is the
// goalkeeper, who cannot replace an outfield player
#[test]
fn player_without_a_valid_substitute_stays_on_the_field() {
    expect_player("Fernandez-Pardo", "has_played", false);
    expect_player("Fernandez-Pardo", "on_field", true);
    expect_play_status("Fernandez-Pardo", "playing");

    expect_player("Suzuki", "has_played", true);
    expect_player("Suzuki", "on_field", false);
    expect_play_status("Suzuki", "benched");
}

#[test]
fn point_sources_of_player_with_bonus() {
    expect_player(
        "Gabriel",
        "point_sources",
        json!([
            { "name": "Minutes played", "stat": "minutes", "amount": 90, "points_total": 2, "fixture": 71 },
            { "name": "Goals scored", "stat": "goals_scored", "amount": 1, "points_total": 6, "fixture": 71 },
            { "name": "Clean sheets", "stat": "clean_sheets", "amount": 1, "points_total": 4, "fixture": 71 },
            { "name": "Bonus", "stat": "bonus", "amount": 3, "points_total": 3, "fixture": 71 },
            { "name": "Defensive Contribution", "stat": "defensive_contribution", "amount": 14, "points_total": 2, "fixture": 71 },
        ]),
    );
}

#[test]
fn point_sources_of_player_with_red_card() {
    expect_player(
        "Palmer",
        "point_sources",
        json!([
            { "name": "Minutes played", "stat": "minutes", "amount": 79, "points_total": 2, "fixture": 73 },
            { "name": "Red cards", "stat": "red_cards", "amount": 1, "points_total": -3, "fixture": 73 },
        ]),
    );
}

#[track_caller]
fn expect_team(team_name: &str, property: &str, expected: impl Into<Value>) {
    expect_team_in(&FINISHED_GW, team_name, property, expected);
}

#[track_caller]
fn expect_player(display_name: &str, property: &str, expected: impl Into<Value>) {
    expect_player_in(&FINISHED_GW, display_name, property, expected);
}

#[track_caller]
fn expect_play_status(display_name: &str, expected: &str) {
    expect_play_status_in(&FINISHED_GW, display_name, expected);
}

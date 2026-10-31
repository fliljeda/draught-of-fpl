// GW 9 mid-gameweek: 7 matches over, 1 in progress and 2 not started, with 5 auto-subs
// projected but not yet made by FPL.

use std::sync::LazyLock;

use serde_json::{json, Value};

use super::*;

// Built once by the first test that uses it, then shared by the others
static MIDWEEK: LazyLock<Value> = LazyLock::new(|| get_table("2026-10-31-midweek"));

// All values are the program's output for this snapshot. FPL's standings are not
// updated during a gameweek, so there is no official total to compare with.

#[test]
fn teams_are_ordered_by_total_points() {
    assert_eq!(MIDWEEK["entries"][0]["team_name"], TEAM_2);
    assert_eq!(MIDWEEK["entries"][1]["team_name"], TEAM_3);
    assert_eq!(MIDWEEK["entries"][2]["team_name"], TEAM_1);
    assert_eq!(MIDWEEK["entries"][3]["team_name"], TEAM_5);
    assert_eq!(MIDWEEK["entries"][4]["team_name"], TEAM_6);
    assert_eq!(MIDWEEK["entries"][5]["team_name"], TEAM_4);
}

#[test]
fn team_total_points() {
    expect_team(TEAM_1, "total_points", 372);
    expect_team(TEAM_2, "total_points", 404);
    expect_team(TEAM_3, "total_points", 400);
    expect_team(TEAM_4, "total_points", 342);
    expect_team(TEAM_5, "total_points", 370);
    expect_team(TEAM_6, "total_points", 368);
}

#[test]
fn team_gameweek_points() {
    expect_team(TEAM_1, "gw_points", 33);
    expect_team(TEAM_2, "gw_points", 28);
    expect_team(TEAM_3, "gw_points", 59);
    expect_team(TEAM_4, "gw_points", 23);
    expect_team(TEAM_5, "gw_points", 11);
    expect_team(TEAM_6, "gw_points", 11);
}

#[test]
fn team_total_projected_points() {
    expect_team(TEAM_1, "total_projected_points", 372);
    expect_team(TEAM_2, "total_projected_points", 404);
    expect_team(TEAM_3, "total_projected_points", 400);
    expect_team(TEAM_4, "total_projected_points", 346);
    expect_team(TEAM_5, "total_projected_points", 383);
    expect_team(TEAM_6, "total_projected_points", 369);
}

#[test]
fn team_gameweek_projected_points() {
    expect_team(TEAM_1, "gw_projected_points", 33);
    expect_team(TEAM_2, "gw_projected_points", 28);
    expect_team(TEAM_3, "gw_projected_points", 59);
    expect_team(TEAM_4, "gw_projected_points", 27);
    expect_team(TEAM_5, "gw_projected_points", 24);
    expect_team(TEAM_6, "gw_projected_points", 12);
}

#[test]
fn team_projected_points_explanation() {
    expect_team(TEAM_1, "projected_points_explanation", json!([]));
    expect_team(TEAM_2, "projected_points_explanation", json!([]));
    expect_team(TEAM_3, "projected_points_explanation", json!([]));
    expect_team(
        TEAM_4,
        "projected_points_explanation",
        json!([
            { "name": "Xhaka", "bonus_points": null, "subbed_points": 4 },
        ]),
    );
    expect_team(
        TEAM_5,
        "projected_points_explanation",
        json!([
            { "name": "Matheus N.", "bonus_points": null, "subbed_points": 2 },
            { "name": "Cunha", "bonus_points": null, "subbed_points": 2 },
            { "name": "De Cuyper", "bonus_points": null, "subbed_points": 9 },
        ]),
    );
    expect_team(
        TEAM_6,
        "projected_points_explanation",
        json!([
            { "name": "Isidor", "bonus_points": null, "subbed_points": 1 },
        ]),
    );
}

#[test]
fn auto_sub_xhaka_replaces_joao_pedro() {
    // João Pedro is 165 and Xhaka is 544
    expect_player(
        "João Pedro",
        "play_status",
        json!({ "type": "subbed_off", "subbed_with": 544 }),
    );
    expect_player(
        "Xhaka",
        "play_status",
        json!({ "type": "subbed_in", "subbed_with": 165 }),
    );

    expect_player("João Pedro", "has_played", false);
    expect_player("Xhaka", "has_played", true);

    // on_field shows FPL's lineup, which has not made this sub yet
    expect_player("João Pedro", "on_field", true);
    expect_player("Xhaka", "on_field", false);
}

#[test]
fn auto_sub_matheus_n_replaces_senesi() {
    // Senesi is 498 and Matheus N. is 389
    expect_player(
        "Senesi",
        "play_status",
        json!({ "type": "subbed_off", "subbed_with": 389 }),
    );
    expect_player(
        "Matheus N.",
        "play_status",
        json!({ "type": "subbed_in", "subbed_with": 498 }),
    );

    expect_player("Senesi", "has_played", false);
    expect_player("Matheus N.", "has_played", true);

    // on_field shows FPL's lineup, which has not made this sub yet
    expect_player("Senesi", "on_field", true);
    expect_player("Matheus N.", "on_field", false);
}

#[test]
fn auto_sub_de_cuyper_replaces_mukiele() {
    // Mukiele is 533 and De Cuyper is 115
    expect_player(
        "Mukiele",
        "play_status",
        json!({ "type": "subbed_off", "subbed_with": 115 }),
    );
    expect_player(
        "De Cuyper",
        "play_status",
        json!({ "type": "subbed_in", "subbed_with": 533 }),
    );

    expect_player("Mukiele", "has_played", false);
    expect_player("De Cuyper", "has_played", true);

    // on_field shows FPL's lineup, which has not made this sub yet
    expect_player("Mukiele", "on_field", true);
    expect_player("De Cuyper", "on_field", false);
}

#[test]
fn auto_sub_cunha_replaces_rashford() {
    // Rashford is 429 and Cunha is 428
    expect_player(
        "Rashford",
        "play_status",
        json!({ "type": "subbed_off", "subbed_with": 428 }),
    );
    expect_player(
        "Cunha",
        "play_status",
        json!({ "type": "subbed_in", "subbed_with": 429 }),
    );

    expect_player("Rashford", "has_played", false);
    expect_player("Cunha", "has_played", true);

    // on_field shows FPL's lineup, which has not made this sub yet
    expect_player("Rashford", "on_field", true);
    expect_player("Cunha", "on_field", false);
}

#[test]
fn auto_sub_isidor_replaces_semenyo() {
    // Semenyo is 397 and Isidor is 553
    expect_player(
        "Semenyo",
        "play_status",
        json!({ "type": "subbed_off", "subbed_with": 553 }),
    );
    expect_player(
        "Isidor",
        "play_status",
        json!({ "type": "subbed_in", "subbed_with": 397 }),
    );

    expect_player("Semenyo", "has_played", false);
    expect_player("Isidor", "has_played", true);

    // on_field shows FPL's lineup, which has not made this sub yet
    expect_player("Semenyo", "on_field", true);
    expect_player("Isidor", "on_field", false);
}

// A goalkeeper can only replace a goalkeeper. These played, but stay on the bench
// and later bench players are subbed in instead
#[test]
fn bench_goalkeeper_is_not_subbed_in_for_an_outfield_player() {
    expect_player("Suzuki", "has_played", true);
    expect_play_status("Suzuki", "benched");

    expect_player("Henderson", "has_played", true);
    expect_play_status("Henderson", "benched");

    expect_player("Verbruggen", "has_played", true);
    expect_play_status("Verbruggen", "benched");
}

// These have not played yet, but their match is still to come
#[test]
fn player_with_an_upcoming_match_is_not_subbed_off() {
    expect_player("Virgil", "has_played", false);
    expect_player("Virgil", "has_upcoming_fixtures", true);
    expect_play_status("Virgil", "playing");

    expect_player("Pickford", "has_played", false);
    expect_player("Pickford", "has_upcoming_fixtures", true);
    expect_play_status("Pickford", "playing");

    expect_player("Raya", "has_played", false);
    expect_player("Raya", "has_upcoming_fixtures", true);
    expect_play_status("Raya", "playing");
}

// N.Jackson's match is in progress
#[test]
fn player_in_a_match_in_progress() {
    expect_player("N.Jackson", "has_played", true);
    expect_player("N.Jackson", "fixtures_finished", false);
    expect_player("N.Jackson", "has_upcoming_fixtures", false);
}

// FPL already includes bonus points in the live data of a match in progress
#[test]
fn point_sources_of_player_in_a_match_in_progress() {
    expect_player(
        "N.Jackson",
        "point_sources",
        json!([
            { "name": "Minutes played", "stat": "minutes", "amount": 63, "points_total": 2, "fixture": 81 },
            { "name": "Goals scored", "stat": "goals_scored", "amount": 1, "points_total": 4, "fixture": 81 },
            { "name": "Bonus", "stat": "bonus", "amount": 3, "points_total": 3, "fixture": 81 },
        ]),
    );
}

#[test]
fn point_sources_of_player_with_bonus() {
    expect_player(
        "McBurnie",
        "point_sources",
        json!([
            { "name": "Minutes played", "stat": "minutes", "amount": 90, "points_total": 2, "fixture": 86 },
            { "name": "Goals scored", "stat": "goals_scored", "amount": 1, "points_total": 4, "fixture": 86 },
            { "name": "Assists", "stat": "assists", "amount": 1, "points_total": 3, "fixture": 86 },
            { "name": "Bonus", "stat": "bonus", "amount": 3, "points_total": 3, "fixture": 86 },
        ]),
    );
}

#[test]
fn point_sources_of_bench_player_with_negative_points() {
    expect_player(
        "Milenković",
        "point_sources",
        json!([
            { "name": "Minutes played", "stat": "minutes", "amount": 90, "points_total": 2, "fixture": 83 },
            { "name": "Goals conceded", "stat": "goals_conceded", "amount": 2, "points_total": -1, "fixture": 83 },
            { "name": "Own goals", "stat": "own_goals", "amount": 1, "points_total": -2, "fixture": 83 },
            { "name": "Yellow cards", "stat": "yellow_cards", "amount": 1, "points_total": -1, "fixture": 83 },
        ]),
    );
}

#[test]
fn player_points() {
    // Team 01
    expect_player("Kelleher", "points", 7);
    expect_player("Davis", "points", 1);
    expect_player("Muharemović", "points", 3);
    expect_player("Virgil", "points", 0);
    expect_player("Manzambi", "points", 6);
    expect_player("Groß", "points", 2);
    expect_player("Rayan", "points", 3);
    expect_player("Barcola", "points", 0);
    expect_player("Isak", "points", 0);
    expect_player("Solanke", "points", 2);
    expect_player("Thiago", "points", 9);
    expect_player("Donnarumma", "points", 2);
    expect_player("Guéhi", "points", 0);
    expect_player("Calafiori", "points", 0);
    expect_player("Wirtz", "points", 0);

    // Team 02
    expect_player("Horníček", "points", 0);
    expect_player("White", "points", 0);
    expect_player("Gvardiol", "points", 2);
    expect_player("Bogle", "points", 1);
    expect_player("Jacquet", "points", 0);
    expect_player("Barnes", "points", 0);
    expect_player("B.Fernandes", "points", 2);
    expect_player("Mbeumo", "points", 1);
    expect_player("McBurnie", "points", 12);
    expect_player("Thomas-Asante", "points", 1);
    expect_player("Evanilson", "points", 9);
    expect_player("A.Becker", "points", 0);
    expect_player("Bruno G.", "points", 0);
    expect_player("Thomas", "points", 0);
    expect_player("Tzolis", "points", 0);

    // Team 03
    expect_player("Kinsky", "points", 6);
    expect_player("Konsa", "points", 0);
    expect_player("Van Hecke", "points", 15);
    expect_player("Ajer", "points", 8);
    expect_player("Kroupi.Jr", "points", 4);
    expect_player("Enzo", "points", 2);
    expect_player("Palmer", "points", 8);
    expect_player("Schade", "points", 3);
    expect_player("Delap", "points", 2);
    expect_player("Brobbey", "points", 2);
    expect_player("N.Jackson", "points", 9);
    expect_player("Trafford", "points", 2);
    expect_player("Cherki", "points", 1);
    expect_player("Vuskovic", "points", 6);
    expect_player("Araujo", "points", 0);

    // Team 04
    expect_player("Tzolakis", "points", 1);
    expect_player("Hall", "points", 0);
    expect_player("Tarkowski", "points", 0);
    expect_player("Maitland-Niles", "points", 0);
    expect_player("Rogers", "points", 9);
    expect_player("Ødegaard", "points", 0);
    expect_player("O.Dango", "points", 6);
    expect_player("Tavernier", "points", 3);
    expect_player("Kostoulas", "points", 2);
    expect_player("Marmoush", "points", 2);
    expect_player("João Pedro", "points", 0);
    expect_player("Suzuki", "points", 7);
    expect_player("Xhaka", "points", 4);
    expect_player("Mykolenko", "points", 0);
    expect_player("Branthwaite", "points", 0);

    // Team 05
    expect_player("Pickford", "points", 0);
    expect_player("Senesi", "points", 0);
    expect_player("Rúben", "points", 2);
    expect_player("Mukiele", "points", 0);
    expect_player("Gibbs-White", "points", 2);
    expect_player("Foden", "points", 1);
    expect_player("Ndiaye", "points", 2);
    expect_player("Rashford", "points", 0);
    expect_player("Barry", "points", 0);
    expect_player("Haaland", "points", 2);
    expect_player("Emersonn", "points", 2);
    expect_player("Henderson", "points", 3);
    expect_player("Matheus N.", "points", 2);
    expect_player("Cunha", "points", 2);
    expect_player("De Cuyper", "points", 9);

    // Team 06
    expect_player("Raya", "points", 0);
    expect_player("Gabriel", "points", 0);
    expect_player("J.Timber", "points", 0);
    expect_player("Ajayi", "points", 2);
    expect_player("Gomez", "points", 3);
    expect_player("Semenyo", "points", 0);
    expect_player("Rice", "points", 0);
    expect_player("Gakpo", "points", 0);
    expect_player("Doku", "points", 4);
    expect_player("Calvert-Lewin", "points", 2);
    expect_player("Havertz", "points", 0);
    expect_player("Verbruggen", "points", 11);
    expect_player("Isidor", "points", 1);
    expect_player("Castagne", "points", 2);
    expect_player("Milenković", "points", -2);
}

#[test]
fn player_projected_points() {
    // Team 01
    expect_player("Kelleher", "projected_points", 7);
    expect_player("Davis", "projected_points", 1);
    expect_player("Muharemović", "projected_points", 3);
    expect_player("Virgil", "projected_points", 0);
    expect_player("Manzambi", "projected_points", 6);
    expect_player("Groß", "projected_points", 2);
    expect_player("Rayan", "projected_points", 3);
    expect_player("Barcola", "projected_points", 0);
    expect_player("Isak", "projected_points", 0);
    expect_player("Solanke", "projected_points", 2);
    expect_player("Thiago", "projected_points", 9);
    expect_player("Donnarumma", "projected_points", 2);
    expect_player("Guéhi", "projected_points", 0);
    expect_player("Calafiori", "projected_points", 0);
    expect_player("Wirtz", "projected_points", 0);

    // Team 02
    expect_player("Horníček", "projected_points", 0);
    expect_player("White", "projected_points", 0);
    expect_player("Gvardiol", "projected_points", 2);
    expect_player("Bogle", "projected_points", 1);
    expect_player("Jacquet", "projected_points", 0);
    expect_player("Barnes", "projected_points", 0);
    expect_player("B.Fernandes", "projected_points", 2);
    expect_player("Mbeumo", "projected_points", 1);
    expect_player("McBurnie", "projected_points", 12);
    expect_player("Thomas-Asante", "projected_points", 1);
    expect_player("Evanilson", "projected_points", 9);
    expect_player("A.Becker", "projected_points", 0);
    expect_player("Bruno G.", "projected_points", 0);
    expect_player("Thomas", "projected_points", 0);
    expect_player("Tzolis", "projected_points", 0);

    // Team 03
    expect_player("Kinsky", "projected_points", 6);
    expect_player("Konsa", "projected_points", 0);
    expect_player("Van Hecke", "projected_points", 15);
    expect_player("Ajer", "projected_points", 8);
    expect_player("Kroupi.Jr", "projected_points", 4);
    expect_player("Enzo", "projected_points", 2);
    expect_player("Palmer", "projected_points", 8);
    expect_player("Schade", "projected_points", 3);
    expect_player("Delap", "projected_points", 2);
    expect_player("Brobbey", "projected_points", 2);
    expect_player("N.Jackson", "projected_points", 9);
    expect_player("Trafford", "projected_points", 2);
    expect_player("Cherki", "projected_points", 1);
    expect_player("Vuskovic", "projected_points", 6);
    expect_player("Araujo", "projected_points", 0);

    // Team 04
    expect_player("Tzolakis", "projected_points", 1);
    expect_player("Hall", "projected_points", 0);
    expect_player("Tarkowski", "projected_points", 0);
    expect_player("Maitland-Niles", "projected_points", 0);
    expect_player("Rogers", "projected_points", 9);
    expect_player("Ødegaard", "projected_points", 0);
    expect_player("O.Dango", "projected_points", 6);
    expect_player("Tavernier", "projected_points", 3);
    expect_player("Kostoulas", "projected_points", 2);
    expect_player("Marmoush", "projected_points", 2);
    expect_player("João Pedro", "projected_points", 0);
    expect_player("Suzuki", "projected_points", 7);
    expect_player("Xhaka", "projected_points", 4);
    expect_player("Mykolenko", "projected_points", 0);
    expect_player("Branthwaite", "projected_points", 0);

    // Team 05
    expect_player("Pickford", "projected_points", 0);
    expect_player("Senesi", "projected_points", 0);
    expect_player("Rúben", "projected_points", 2);
    expect_player("Mukiele", "projected_points", 0);
    expect_player("Gibbs-White", "projected_points", 2);
    expect_player("Foden", "projected_points", 1);
    expect_player("Ndiaye", "projected_points", 2);
    expect_player("Rashford", "projected_points", 0);
    expect_player("Barry", "projected_points", 0);
    expect_player("Haaland", "projected_points", 2);
    expect_player("Emersonn", "projected_points", 2);
    expect_player("Henderson", "projected_points", 3);
    expect_player("Matheus N.", "projected_points", 2);
    expect_player("Cunha", "projected_points", 2);
    expect_player("De Cuyper", "projected_points", 9);

    // Team 06
    expect_player("Raya", "projected_points", 0);
    expect_player("Gabriel", "projected_points", 0);
    expect_player("J.Timber", "projected_points", 0);
    expect_player("Ajayi", "projected_points", 2);
    expect_player("Gomez", "projected_points", 3);
    expect_player("Semenyo", "projected_points", 0);
    expect_player("Rice", "projected_points", 0);
    expect_player("Gakpo", "projected_points", 0);
    expect_player("Doku", "projected_points", 4);
    expect_player("Calvert-Lewin", "projected_points", 2);
    expect_player("Havertz", "projected_points", 0);
    expect_player("Verbruggen", "projected_points", 11);
    expect_player("Isidor", "projected_points", 1);
    expect_player("Castagne", "projected_points", 2);
    expect_player("Milenković", "projected_points", -2);
}

#[test]
fn player_bps() {
    // Team 01
    expect_player("Kelleher", "bps", 24);
    expect_player("Davis", "bps", 4);
    expect_player("Muharemović", "bps", 9);
    expect_player("Virgil", "bps", 0);
    expect_player("Manzambi", "bps", 24);
    expect_player("Groß", "bps", 5);
    expect_player("Rayan", "bps", 7);
    expect_player("Barcola", "bps", 0);
    expect_player("Isak", "bps", 0);
    expect_player("Solanke", "bps", 13);
    expect_player("Thiago", "bps", 37);
    expect_player("Donnarumma", "bps", 16);
    expect_player("Guéhi", "bps", 0);
    expect_player("Calafiori", "bps", 0);
    expect_player("Wirtz", "bps", 0);

    // Team 02
    expect_player("Horníček", "bps", 0);
    expect_player("White", "bps", 0);
    expect_player("Gvardiol", "bps", 10);
    expect_player("Bogle", "bps", 5);
    expect_player("Jacquet", "bps", 0);
    expect_player("Barnes", "bps", 0);
    expect_player("B.Fernandes", "bps", 16);
    expect_player("Mbeumo", "bps", 15);
    expect_player("McBurnie", "bps", 46);
    expect_player("Thomas-Asante", "bps", 3);
    expect_player("Evanilson", "bps", 42);
    expect_player("A.Becker", "bps", 0);
    expect_player("Bruno G.", "bps", 0);
    expect_player("Thomas", "bps", 0);
    expect_player("Tzolis", "bps", 0);

    // Team 03
    expect_player("Kinsky", "bps", 23);
    expect_player("Konsa", "bps", 0);
    expect_player("Van Hecke", "bps", 38);
    expect_player("Ajer", "bps", 23);
    expect_player("Kroupi.Jr", "bps", 21);
    expect_player("Enzo", "bps", 11);
    expect_player("Palmer", "bps", 37);
    expect_player("Schade", "bps", 9);
    expect_player("Delap", "bps", 0);
    expect_player("Brobbey", "bps", 6);
    expect_player("N.Jackson", "bps", 40);
    expect_player("Trafford", "bps", 6);
    expect_player("Cherki", "bps", 10);
    expect_player("Vuskovic", "bps", 25);
    expect_player("Araujo", "bps", 0);

    // Team 04
    expect_player("Tzolakis", "bps", 4);
    expect_player("Hall", "bps", 0);
    expect_player("Tarkowski", "bps", 0);
    expect_player("Maitland-Niles", "bps", 0);
    expect_player("Rogers", "bps", 38);
    expect_player("Ødegaard", "bps", 0);
    expect_player("O.Dango", "bps", 22);
    expect_player("Tavernier", "bps", 12);
    expect_player("Kostoulas", "bps", 9);
    expect_player("Marmoush", "bps", 16);
    expect_player("João Pedro", "bps", 0);
    expect_player("Suzuki", "bps", 25);
    expect_player("Xhaka", "bps", 19);
    expect_player("Mykolenko", "bps", 0);
    expect_player("Branthwaite", "bps", 0);

    // Team 05
    expect_player("Pickford", "bps", 0);
    expect_player("Senesi", "bps", 0);
    expect_player("Rúben", "bps", 8);
    expect_player("Mukiele", "bps", 0);
    expect_player("Gibbs-White", "bps", 10);
    expect_player("Foden", "bps", 3);
    expect_player("Ndiaye", "bps", 12);
    expect_player("Rashford", "bps", 0);
    expect_player("Barry", "bps", 0);
    expect_player("Haaland", "bps", 6);
    expect_player("Emersonn", "bps", 5);
    expect_player("Henderson", "bps", 22);
    expect_player("Matheus N.", "bps", 9);
    expect_player("Cunha", "bps", 11);
    expect_player("De Cuyper", "bps", 30);

    // Team 06
    expect_player("Raya", "bps", 0);
    expect_player("Gabriel", "bps", 0);
    expect_player("J.Timber", "bps", 0);
    expect_player("Ajayi", "bps", 1);
    expect_player("Gomez", "bps", 8);
    expect_player("Semenyo", "bps", 0);
    expect_player("Rice", "bps", 0);
    expect_player("Gakpo", "bps", 0);
    expect_player("Doku", "bps", 28);
    expect_player("Calvert-Lewin", "bps", 5);
    expect_player("Havertz", "bps", 0);
    expect_player("Verbruggen", "bps", 46);
    expect_player("Isidor", "bps", 3);
    expect_player("Castagne", "bps", 9);
    expect_player("Milenković", "bps", 0);
}

#[test]
fn player_on_field() {
    // Team 01
    expect_player("Kelleher", "on_field", true);
    expect_player("Davis", "on_field", true);
    expect_player("Muharemović", "on_field", true);
    expect_player("Virgil", "on_field", true);
    expect_player("Manzambi", "on_field", true);
    expect_player("Groß", "on_field", true);
    expect_player("Rayan", "on_field", true);
    expect_player("Barcola", "on_field", true);
    expect_player("Isak", "on_field", true);
    expect_player("Solanke", "on_field", true);
    expect_player("Thiago", "on_field", true);
    expect_player("Donnarumma", "on_field", false);
    expect_player("Guéhi", "on_field", false);
    expect_player("Calafiori", "on_field", false);
    expect_player("Wirtz", "on_field", false);

    // Team 02
    expect_player("Horníček", "on_field", true);
    expect_player("White", "on_field", true);
    expect_player("Gvardiol", "on_field", true);
    expect_player("Bogle", "on_field", true);
    expect_player("Jacquet", "on_field", true);
    expect_player("Barnes", "on_field", true);
    expect_player("B.Fernandes", "on_field", true);
    expect_player("Mbeumo", "on_field", true);
    expect_player("McBurnie", "on_field", true);
    expect_player("Thomas-Asante", "on_field", true);
    expect_player("Evanilson", "on_field", true);
    expect_player("A.Becker", "on_field", false);
    expect_player("Bruno G.", "on_field", false);
    expect_player("Thomas", "on_field", false);
    expect_player("Tzolis", "on_field", false);

    // Team 03
    expect_player("Kinsky", "on_field", true);
    expect_player("Konsa", "on_field", true);
    expect_player("Van Hecke", "on_field", true);
    expect_player("Ajer", "on_field", true);
    expect_player("Kroupi.Jr", "on_field", true);
    expect_player("Enzo", "on_field", true);
    expect_player("Palmer", "on_field", true);
    expect_player("Schade", "on_field", true);
    expect_player("Delap", "on_field", true);
    expect_player("Brobbey", "on_field", true);
    expect_player("N.Jackson", "on_field", true);
    expect_player("Trafford", "on_field", false);
    expect_player("Cherki", "on_field", false);
    expect_player("Vuskovic", "on_field", false);
    expect_player("Araujo", "on_field", false);

    // Team 04
    expect_player("Tzolakis", "on_field", true);
    expect_player("Hall", "on_field", true);
    expect_player("Tarkowski", "on_field", true);
    expect_player("Maitland-Niles", "on_field", true);
    expect_player("Rogers", "on_field", true);
    expect_player("Ødegaard", "on_field", true);
    expect_player("O.Dango", "on_field", true);
    expect_player("Tavernier", "on_field", true);
    expect_player("Kostoulas", "on_field", true);
    expect_player("Marmoush", "on_field", true);
    expect_player("João Pedro", "on_field", true);
    expect_player("Suzuki", "on_field", false);
    expect_player("Xhaka", "on_field", false);
    expect_player("Mykolenko", "on_field", false);
    expect_player("Branthwaite", "on_field", false);

    // Team 05
    expect_player("Pickford", "on_field", true);
    expect_player("Senesi", "on_field", true);
    expect_player("Rúben", "on_field", true);
    expect_player("Mukiele", "on_field", true);
    expect_player("Gibbs-White", "on_field", true);
    expect_player("Foden", "on_field", true);
    expect_player("Ndiaye", "on_field", true);
    expect_player("Rashford", "on_field", true);
    expect_player("Barry", "on_field", true);
    expect_player("Haaland", "on_field", true);
    expect_player("Emersonn", "on_field", true);
    expect_player("Henderson", "on_field", false);
    expect_player("Matheus N.", "on_field", false);
    expect_player("Cunha", "on_field", false);
    expect_player("De Cuyper", "on_field", false);

    // Team 06
    expect_player("Raya", "on_field", true);
    expect_player("Gabriel", "on_field", true);
    expect_player("J.Timber", "on_field", true);
    expect_player("Ajayi", "on_field", true);
    expect_player("Gomez", "on_field", true);
    expect_player("Semenyo", "on_field", true);
    expect_player("Rice", "on_field", true);
    expect_player("Gakpo", "on_field", true);
    expect_player("Doku", "on_field", true);
    expect_player("Calvert-Lewin", "on_field", true);
    expect_player("Havertz", "on_field", true);
    expect_player("Verbruggen", "on_field", false);
    expect_player("Isidor", "on_field", false);
    expect_player("Castagne", "on_field", false);
    expect_player("Milenković", "on_field", false);
}

#[test]
fn player_has_played() {
    // Team 01
    expect_player("Kelleher", "has_played", true);
    expect_player("Davis", "has_played", true);
    expect_player("Muharemović", "has_played", true);
    expect_player("Virgil", "has_played", false);
    expect_player("Manzambi", "has_played", true);
    expect_player("Groß", "has_played", true);
    expect_player("Rayan", "has_played", true);
    expect_player("Barcola", "has_played", false);
    expect_player("Isak", "has_played", false);
    expect_player("Solanke", "has_played", true);
    expect_player("Thiago", "has_played", true);
    expect_player("Donnarumma", "has_played", true);
    expect_player("Guéhi", "has_played", false);
    expect_player("Calafiori", "has_played", false);
    expect_player("Wirtz", "has_played", false);

    // Team 02
    expect_player("Horníček", "has_played", false);
    expect_player("White", "has_played", false);
    expect_player("Gvardiol", "has_played", true);
    expect_player("Bogle", "has_played", true);
    expect_player("Jacquet", "has_played", false);
    expect_player("Barnes", "has_played", false);
    expect_player("B.Fernandes", "has_played", true);
    expect_player("Mbeumo", "has_played", true);
    expect_player("McBurnie", "has_played", true);
    expect_player("Thomas-Asante", "has_played", true);
    expect_player("Evanilson", "has_played", true);
    expect_player("A.Becker", "has_played", false);
    expect_player("Bruno G.", "has_played", false);
    expect_player("Thomas", "has_played", false);
    expect_player("Tzolis", "has_played", false);

    // Team 03
    expect_player("Kinsky", "has_played", true);
    expect_player("Konsa", "has_played", false);
    expect_player("Van Hecke", "has_played", true);
    expect_player("Ajer", "has_played", true);
    expect_player("Kroupi.Jr", "has_played", true);
    expect_player("Enzo", "has_played", true);
    expect_player("Palmer", "has_played", true);
    expect_player("Schade", "has_played", true);
    expect_player("Delap", "has_played", true);
    expect_player("Brobbey", "has_played", true);
    expect_player("N.Jackson", "has_played", true);
    expect_player("Trafford", "has_played", true);
    expect_player("Cherki", "has_played", true);
    expect_player("Vuskovic", "has_played", true);
    expect_player("Araujo", "has_played", false);

    // Team 04
    expect_player("Tzolakis", "has_played", true);
    expect_player("Hall", "has_played", false);
    expect_player("Tarkowski", "has_played", false);
    expect_player("Maitland-Niles", "has_played", false);
    expect_player("Rogers", "has_played", true);
    expect_player("Ødegaard", "has_played", false);
    expect_player("O.Dango", "has_played", true);
    expect_player("Tavernier", "has_played", true);
    expect_player("Kostoulas", "has_played", true);
    expect_player("Marmoush", "has_played", true);
    expect_player("João Pedro", "has_played", false);
    expect_player("Suzuki", "has_played", true);
    expect_player("Xhaka", "has_played", true);
    expect_player("Mykolenko", "has_played", false);
    expect_player("Branthwaite", "has_played", false);

    // Team 05
    expect_player("Pickford", "has_played", false);
    expect_player("Senesi", "has_played", false);
    expect_player("Rúben", "has_played", true);
    expect_player("Mukiele", "has_played", false);
    expect_player("Gibbs-White", "has_played", true);
    expect_player("Foden", "has_played", true);
    expect_player("Ndiaye", "has_played", true);
    expect_player("Rashford", "has_played", false);
    expect_player("Barry", "has_played", false);
    expect_player("Haaland", "has_played", true);
    expect_player("Emersonn", "has_played", true);
    expect_player("Henderson", "has_played", true);
    expect_player("Matheus N.", "has_played", true);
    expect_player("Cunha", "has_played", true);
    expect_player("De Cuyper", "has_played", true);

    // Team 06
    expect_player("Raya", "has_played", false);
    expect_player("Gabriel", "has_played", false);
    expect_player("J.Timber", "has_played", false);
    expect_player("Ajayi", "has_played", true);
    expect_player("Gomez", "has_played", true);
    expect_player("Semenyo", "has_played", false);
    expect_player("Rice", "has_played", false);
    expect_player("Gakpo", "has_played", false);
    expect_player("Doku", "has_played", true);
    expect_player("Calvert-Lewin", "has_played", true);
    expect_player("Havertz", "has_played", false);
    expect_player("Verbruggen", "has_played", true);
    expect_player("Isidor", "has_played", true);
    expect_player("Castagne", "has_played", true);
    expect_player("Milenković", "has_played", true);
}

#[test]
fn player_play_status() {
    // Team 01
    expect_play_status("Kelleher", "playing");
    expect_play_status("Davis", "playing");
    expect_play_status("Muharemović", "playing");
    expect_play_status("Virgil", "playing");
    expect_play_status("Manzambi", "playing");
    expect_play_status("Groß", "playing");
    expect_play_status("Rayan", "playing");
    expect_play_status("Barcola", "playing");
    expect_play_status("Isak", "playing");
    expect_play_status("Solanke", "playing");
    expect_play_status("Thiago", "playing");
    expect_play_status("Donnarumma", "benched");
    expect_play_status("Guéhi", "benched");
    expect_play_status("Calafiori", "benched");
    expect_play_status("Wirtz", "benched");

    // Team 02
    expect_play_status("Horníček", "playing");
    expect_play_status("White", "playing");
    expect_play_status("Gvardiol", "playing");
    expect_play_status("Bogle", "playing");
    expect_play_status("Jacquet", "playing");
    expect_play_status("Barnes", "playing");
    expect_play_status("B.Fernandes", "playing");
    expect_play_status("Mbeumo", "playing");
    expect_play_status("McBurnie", "playing");
    expect_play_status("Thomas-Asante", "playing");
    expect_play_status("Evanilson", "playing");
    expect_play_status("A.Becker", "benched");
    expect_play_status("Bruno G.", "benched");
    expect_play_status("Thomas", "benched");
    expect_play_status("Tzolis", "benched");

    // Team 03
    expect_play_status("Kinsky", "playing");
    expect_play_status("Konsa", "playing");
    expect_play_status("Van Hecke", "playing");
    expect_play_status("Ajer", "playing");
    expect_play_status("Kroupi.Jr", "playing");
    expect_play_status("Enzo", "playing");
    expect_play_status("Palmer", "playing");
    expect_play_status("Schade", "playing");
    expect_play_status("Delap", "playing");
    expect_play_status("Brobbey", "playing");
    expect_play_status("N.Jackson", "playing");
    expect_play_status("Trafford", "benched");
    expect_play_status("Cherki", "benched");
    expect_play_status("Vuskovic", "benched");
    expect_play_status("Araujo", "benched");

    // Team 04
    expect_play_status("Tzolakis", "playing");
    expect_play_status("Hall", "playing");
    expect_play_status("Tarkowski", "playing");
    expect_play_status("Maitland-Niles", "playing");
    expect_play_status("Rogers", "playing");
    expect_play_status("Ødegaard", "playing");
    expect_play_status("O.Dango", "playing");
    expect_play_status("Tavernier", "playing");
    expect_play_status("Kostoulas", "playing");
    expect_play_status("Marmoush", "playing");
    expect_play_status("João Pedro", "subbed_off");
    expect_play_status("Suzuki", "benched");
    expect_play_status("Xhaka", "subbed_in");
    expect_play_status("Mykolenko", "benched");
    expect_play_status("Branthwaite", "benched");

    // Team 05
    expect_play_status("Pickford", "playing");
    expect_play_status("Senesi", "subbed_off");
    expect_play_status("Rúben", "playing");
    expect_play_status("Mukiele", "subbed_off");
    expect_play_status("Gibbs-White", "playing");
    expect_play_status("Foden", "playing");
    expect_play_status("Ndiaye", "playing");
    expect_play_status("Rashford", "subbed_off");
    expect_play_status("Barry", "playing");
    expect_play_status("Haaland", "playing");
    expect_play_status("Emersonn", "playing");
    expect_play_status("Henderson", "benched");
    expect_play_status("Matheus N.", "subbed_in");
    expect_play_status("Cunha", "subbed_in");
    expect_play_status("De Cuyper", "subbed_in");

    // Team 06
    expect_play_status("Raya", "playing");
    expect_play_status("Gabriel", "playing");
    expect_play_status("J.Timber", "playing");
    expect_play_status("Ajayi", "playing");
    expect_play_status("Gomez", "playing");
    expect_play_status("Semenyo", "subbed_off");
    expect_play_status("Rice", "playing");
    expect_play_status("Gakpo", "playing");
    expect_play_status("Doku", "playing");
    expect_play_status("Calvert-Lewin", "playing");
    expect_play_status("Havertz", "playing");
    expect_play_status("Verbruggen", "benched");
    expect_play_status("Isidor", "subbed_in");
    expect_play_status("Castagne", "benched");
    expect_play_status("Milenković", "benched");
}

#[test]
fn player_fixtures_finished() {
    // Team 01
    expect_player("Kelleher", "fixtures_finished", true);
    expect_player("Davis", "fixtures_finished", true);
    expect_player("Muharemović", "fixtures_finished", true);
    expect_player("Virgil", "fixtures_finished", false);
    expect_player("Manzambi", "fixtures_finished", false);
    expect_player("Groß", "fixtures_finished", true);
    expect_player("Rayan", "fixtures_finished", true);
    expect_player("Barcola", "fixtures_finished", false);
    expect_player("Isak", "fixtures_finished", false);
    expect_player("Solanke", "fixtures_finished", true);
    expect_player("Thiago", "fixtures_finished", true);
    expect_player("Donnarumma", "fixtures_finished", true);
    expect_player("Guéhi", "fixtures_finished", true);
    expect_player("Calafiori", "fixtures_finished", false);
    expect_player("Wirtz", "fixtures_finished", false);

    // Team 02
    expect_player("Horníček", "fixtures_finished", false);
    expect_player("White", "fixtures_finished", false);
    expect_player("Gvardiol", "fixtures_finished", true);
    expect_player("Bogle", "fixtures_finished", true);
    expect_player("Jacquet", "fixtures_finished", false);
    expect_player("Barnes", "fixtures_finished", false);
    expect_player("B.Fernandes", "fixtures_finished", true);
    expect_player("Mbeumo", "fixtures_finished", true);
    expect_player("McBurnie", "fixtures_finished", true);
    expect_player("Thomas-Asante", "fixtures_finished", true);
    expect_player("Evanilson", "fixtures_finished", true);
    expect_player("A.Becker", "fixtures_finished", false);
    expect_player("Bruno G.", "fixtures_finished", false);
    expect_player("Thomas", "fixtures_finished", true);
    expect_player("Tzolis", "fixtures_finished", false);

    // Team 03
    expect_player("Kinsky", "fixtures_finished", true);
    expect_player("Konsa", "fixtures_finished", false);
    expect_player("Van Hecke", "fixtures_finished", true);
    expect_player("Ajer", "fixtures_finished", true);
    expect_player("Kroupi.Jr", "fixtures_finished", true);
    expect_player("Enzo", "fixtures_finished", true);
    expect_player("Palmer", "fixtures_finished", true);
    expect_player("Schade", "fixtures_finished", true);
    expect_player("Delap", "fixtures_finished", true);
    expect_player("Brobbey", "fixtures_finished", true);
    expect_player("N.Jackson", "fixtures_finished", false);
    expect_player("Trafford", "fixtures_finished", true);
    expect_player("Cherki", "fixtures_finished", true);
    expect_player("Vuskovic", "fixtures_finished", true);
    expect_player("Araujo", "fixtures_finished", false);

    // Team 04
    expect_player("Tzolakis", "fixtures_finished", true);
    expect_player("Hall", "fixtures_finished", false);
    expect_player("Tarkowski", "fixtures_finished", false);
    expect_player("Maitland-Niles", "fixtures_finished", false);
    expect_player("Rogers", "fixtures_finished", true);
    expect_player("Ødegaard", "fixtures_finished", false);
    expect_player("O.Dango", "fixtures_finished", true);
    expect_player("Tavernier", "fixtures_finished", true);
    expect_player("Kostoulas", "fixtures_finished", true);
    expect_player("Marmoush", "fixtures_finished", true);
    expect_player("João Pedro", "fixtures_finished", true);
    expect_player("Suzuki", "fixtures_finished", false);
    expect_player("Xhaka", "fixtures_finished", true);
    expect_player("Mykolenko", "fixtures_finished", false);
    expect_player("Branthwaite", "fixtures_finished", false);

    // Team 05
    expect_player("Pickford", "fixtures_finished", false);
    expect_player("Senesi", "fixtures_finished", true);
    expect_player("Rúben", "fixtures_finished", true);
    expect_player("Mukiele", "fixtures_finished", true);
    expect_player("Gibbs-White", "fixtures_finished", true);
    expect_player("Foden", "fixtures_finished", true);
    expect_player("Ndiaye", "fixtures_finished", true);
    expect_player("Rashford", "fixtures_finished", true);
    expect_player("Barry", "fixtures_finished", false);
    expect_player("Haaland", "fixtures_finished", true);
    expect_player("Emersonn", "fixtures_finished", true);
    expect_player("Henderson", "fixtures_finished", true);
    expect_player("Matheus N.", "fixtures_finished", true);
    expect_player("Cunha", "fixtures_finished", true);
    expect_player("De Cuyper", "fixtures_finished", true);

    // Team 06
    expect_player("Raya", "fixtures_finished", false);
    expect_player("Gabriel", "fixtures_finished", false);
    expect_player("J.Timber", "fixtures_finished", false);
    expect_player("Ajayi", "fixtures_finished", true);
    expect_player("Gomez", "fixtures_finished", true);
    expect_player("Semenyo", "fixtures_finished", true);
    expect_player("Rice", "fixtures_finished", false);
    expect_player("Gakpo", "fixtures_finished", false);
    expect_player("Doku", "fixtures_finished", true);
    expect_player("Calvert-Lewin", "fixtures_finished", true);
    expect_player("Havertz", "fixtures_finished", false);
    expect_player("Verbruggen", "fixtures_finished", true);
    expect_player("Isidor", "fixtures_finished", true);
    expect_player("Castagne", "fixtures_finished", false);
    expect_player("Milenković", "fixtures_finished", true);
}

#[test]
fn player_has_upcoming_fixtures() {
    // Team 01
    expect_player("Kelleher", "has_upcoming_fixtures", false);
    expect_player("Davis", "has_upcoming_fixtures", false);
    expect_player("Muharemović", "has_upcoming_fixtures", false);
    expect_player("Virgil", "has_upcoming_fixtures", true);
    expect_player("Manzambi", "has_upcoming_fixtures", false);
    expect_player("Groß", "has_upcoming_fixtures", false);
    expect_player("Rayan", "has_upcoming_fixtures", false);
    expect_player("Barcola", "has_upcoming_fixtures", true);
    expect_player("Isak", "has_upcoming_fixtures", true);
    expect_player("Solanke", "has_upcoming_fixtures", false);
    expect_player("Thiago", "has_upcoming_fixtures", false);
    expect_player("Donnarumma", "has_upcoming_fixtures", false);
    expect_player("Guéhi", "has_upcoming_fixtures", false);
    expect_player("Calafiori", "has_upcoming_fixtures", true);
    expect_player("Wirtz", "has_upcoming_fixtures", true);

    // Team 02
    expect_player("Horníček", "has_upcoming_fixtures", true);
    expect_player("White", "has_upcoming_fixtures", true);
    expect_player("Gvardiol", "has_upcoming_fixtures", false);
    expect_player("Bogle", "has_upcoming_fixtures", false);
    expect_player("Jacquet", "has_upcoming_fixtures", true);
    expect_player("Barnes", "has_upcoming_fixtures", true);
    expect_player("B.Fernandes", "has_upcoming_fixtures", false);
    expect_player("Mbeumo", "has_upcoming_fixtures", false);
    expect_player("McBurnie", "has_upcoming_fixtures", false);
    expect_player("Thomas-Asante", "has_upcoming_fixtures", false);
    expect_player("Evanilson", "has_upcoming_fixtures", false);
    expect_player("A.Becker", "has_upcoming_fixtures", true);
    expect_player("Bruno G.", "has_upcoming_fixtures", true);
    expect_player("Thomas", "has_upcoming_fixtures", false);
    expect_player("Tzolis", "has_upcoming_fixtures", true);

    // Team 03
    expect_player("Kinsky", "has_upcoming_fixtures", false);
    expect_player("Konsa", "has_upcoming_fixtures", true);
    expect_player("Van Hecke", "has_upcoming_fixtures", false);
    expect_player("Ajer", "has_upcoming_fixtures", false);
    expect_player("Kroupi.Jr", "has_upcoming_fixtures", false);
    expect_player("Enzo", "has_upcoming_fixtures", false);
    expect_player("Palmer", "has_upcoming_fixtures", false);
    expect_player("Schade", "has_upcoming_fixtures", false);
    expect_player("Delap", "has_upcoming_fixtures", false);
    expect_player("Brobbey", "has_upcoming_fixtures", false);
    expect_player("N.Jackson", "has_upcoming_fixtures", false);
    expect_player("Trafford", "has_upcoming_fixtures", false);
    expect_player("Cherki", "has_upcoming_fixtures", false);
    expect_player("Vuskovic", "has_upcoming_fixtures", false);
    expect_player("Araujo", "has_upcoming_fixtures", true);

    // Team 04
    expect_player("Tzolakis", "has_upcoming_fixtures", false);
    expect_player("Hall", "has_upcoming_fixtures", true);
    expect_player("Tarkowski", "has_upcoming_fixtures", true);
    expect_player("Maitland-Niles", "has_upcoming_fixtures", true);
    expect_player("Rogers", "has_upcoming_fixtures", false);
    expect_player("Ødegaard", "has_upcoming_fixtures", true);
    expect_player("O.Dango", "has_upcoming_fixtures", false);
    expect_player("Tavernier", "has_upcoming_fixtures", false);
    expect_player("Kostoulas", "has_upcoming_fixtures", false);
    expect_player("Marmoush", "has_upcoming_fixtures", false);
    expect_player("João Pedro", "has_upcoming_fixtures", false);
    expect_player("Suzuki", "has_upcoming_fixtures", false);
    expect_player("Xhaka", "has_upcoming_fixtures", false);
    expect_player("Mykolenko", "has_upcoming_fixtures", true);
    expect_player("Branthwaite", "has_upcoming_fixtures", true);

    // Team 05
    expect_player("Pickford", "has_upcoming_fixtures", true);
    expect_player("Senesi", "has_upcoming_fixtures", false);
    expect_player("Rúben", "has_upcoming_fixtures", false);
    expect_player("Mukiele", "has_upcoming_fixtures", false);
    expect_player("Gibbs-White", "has_upcoming_fixtures", false);
    expect_player("Foden", "has_upcoming_fixtures", false);
    expect_player("Ndiaye", "has_upcoming_fixtures", false);
    expect_player("Rashford", "has_upcoming_fixtures", false);
    expect_player("Barry", "has_upcoming_fixtures", true);
    expect_player("Haaland", "has_upcoming_fixtures", false);
    expect_player("Emersonn", "has_upcoming_fixtures", false);
    expect_player("Henderson", "has_upcoming_fixtures", false);
    expect_player("Matheus N.", "has_upcoming_fixtures", false);
    expect_player("Cunha", "has_upcoming_fixtures", false);
    expect_player("De Cuyper", "has_upcoming_fixtures", false);

    // Team 06
    expect_player("Raya", "has_upcoming_fixtures", true);
    expect_player("Gabriel", "has_upcoming_fixtures", true);
    expect_player("J.Timber", "has_upcoming_fixtures", true);
    expect_player("Ajayi", "has_upcoming_fixtures", false);
    expect_player("Gomez", "has_upcoming_fixtures", false);
    expect_player("Semenyo", "has_upcoming_fixtures", false);
    expect_player("Rice", "has_upcoming_fixtures", true);
    expect_player("Gakpo", "has_upcoming_fixtures", true);
    expect_player("Doku", "has_upcoming_fixtures", false);
    expect_player("Calvert-Lewin", "has_upcoming_fixtures", false);
    expect_player("Havertz", "has_upcoming_fixtures", true);
    expect_player("Verbruggen", "has_upcoming_fixtures", false);
    expect_player("Isidor", "has_upcoming_fixtures", false);
    expect_player("Castagne", "has_upcoming_fixtures", false);
    expect_player("Milenković", "has_upcoming_fixtures", false);
}

#[track_caller]
fn expect_team(team_name: &str, property: &str, expected: impl Into<Value>) {
    expect_team_in(&MIDWEEK, team_name, property, expected);
}

#[track_caller]
fn expect_player(display_name: &str, property: &str, expected: impl Into<Value>) {
    expect_player_in(&MIDWEEK, display_name, property, expected);
}

#[track_caller]
fn expect_play_status(display_name: &str, expected: &str) {
    expect_play_status_in(&MIDWEEK, display_name, expected);
}

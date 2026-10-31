// Tests of the whole flow on saved FPL responses:
// snapshot files -> client -> fetcher -> computer -> router -> JSON
//
// Each snapshot has its own file of tests. The helpers they share are here.

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::Value;
use tower::ServiceExt;

use crate::client::Client;
use crate::{build_router, computer, fetcher, initializer, AppState};

mod finished_gw;
mod midweek;

const LEAGUE_ID: u32 = 26952;

const TEAM_1: &str = "Team 01";
const TEAM_2: &str = "Team 02";
const TEAM_3: &str = "Team 03";
const TEAM_4: &str = "Team 04";
const TEAM_5: &str = "Team 05";
const TEAM_6: &str = "Team 06";

// The failure message names the team and the value, and #[track_caller] makes
// the reported line the failing expect line in the test
#[track_caller]
fn expect_team_in(table: &Value, team_name: &str, property: &str, expected: impl Into<Value>) {
    let team = table["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|team| team["team_name"] == team_name)
        .unwrap_or_else(|| panic!("{team_name} is not in the table"));

    assert_eq!(team[property], expected.into(), "{team_name}: {property}");
}

#[track_caller]
fn expect_player_in(table: &Value, display_name: &str, property: &str, expected: impl Into<Value>) {
    assert_eq!(
        find_player(table, display_name)[property],
        expected.into(),
        "{display_name}: {property}"
    );
}

#[track_caller]
fn expect_play_status_in(table: &Value, display_name: &str, expected: &str) {
    assert_eq!(
        find_player(table, display_name)["play_status"]["type"],
        expected,
        "{display_name}: play_status"
    );
}

// Each player is owned by one team in a draft league, so names are unique
#[track_caller]
fn find_player<'a>(table: &'a Value, display_name: &str) -> &'a Value {
    table["entries"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|team| team["players"].as_array().unwrap())
        .find(|player| player["display_name"] == display_name)
        .unwrap_or_else(|| panic!("{display_name} is not in the table"))
}

// Starts the app on a snapshot as main does, without the background tasks, and
// returns the response of GET /table
fn get_table(snapshot: &str) -> Value {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let root = env!("CARGO_MANIFEST_DIR");
        let api = format!("{root}/tests/snapshots/{snapshot}/api");
        let client = Client::new_local(Some(api)).unwrap();

        let context = initializer::initialize_app_context(&client, LEAGUE_ID).await;
        let endpoints = fetcher::fetch_and_initialize_endpoints(&client, context).await;
        let table = computer::compute_new_league_table(endpoints.clone()).unwrap();

        let state = AppState {
            endpoints: Arc::new(RwLock::new(endpoints)),
            table: Arc::new(RwLock::new(table)),
        };
        let app = build_router(state, format!("{root}/www/vue"));

        let request = Request::get("/table").body(Body::empty()).unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&body).unwrap()
    })
}

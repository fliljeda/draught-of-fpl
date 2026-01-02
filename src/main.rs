use std::ops::Deref;
use std::sync::{Arc, RwLock};

use axum::{
    extract::Path, extract::State, http::StatusCode, response::IntoResponse, routing::get, Router,
};
use clap::Parser;
use tower_http::cors::CorsLayer;

pub use initializer::AppContext;

use crate::client::Client;
use crate::storage::{FplEndpoints, LeagueTable};
use tracing_subscriber;

mod client;
mod computer;
mod fetcher;
mod initializer;
mod propcomp;
mod storage;
mod structs;

#[derive(Parser)]
#[command(name = "Draught of FPL")]
#[command(about = "Fantasy Premier League Draft League Proxy Server", long_about = None)]
struct Cli {
    /// Path to configuration file (TOML format). If not provided, uses environment variables.
    #[arg(short = 'f', long)]
    config_source: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    endpoints: Arc<RwLock<FplEndpoints>>,
    table: Arc<RwLock<LeagueTable>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let app_config = initializer::AppConfig::initialize(cli.config_source);

    let client = match app_config.local_fetch {
        Some(true) => Client::new_local(app_config.local_url.clone()).unwrap(),
        Some(false) | None => Client::new().unwrap(),
    };

    let league_id = app_config.league_id;

    let app_context = Arc::new(initializer::initialize_app_context(&client, league_id).await);

    let endpoints =
        fetcher::fetch_and_initialize_endpoints(&client, app_context.deref().clone()).await;

    let initialize_table_endpoints = endpoints.clone();
    let table = computer::compute_new_league_table(initialize_table_endpoints)
        .expect("Failed to compute league table in main");

    let endpoints = Arc::new(RwLock::new(endpoints));
    let endpoints_fetch_clone = Arc::clone(&endpoints);
    let endpoints_compute_clone = Arc::clone(&endpoints);

    let table = Arc::new(RwLock::new(table));
    let table_compute_clone = Arc::clone(&table);

    let state = AppState {
        endpoints: Arc::clone(&endpoints),
        table: Arc::clone(&table),
    };

    tokio::spawn(fetcher::endpoint_cache_fetcher(
        client,
        endpoints_fetch_clone,
        app_context,
    ));
    tokio::spawn(computer::league_table_computer(
        table_compute_clone,
        endpoints_compute_clone,
    ));

    let frontend = tower_http::services::ServeDir::new(
        app_config
            .asset_path
            .clone()
            .unwrap_or_else(|| "./www/vue".into()),
    );
    // Build the router with CORS middleware
    let app = Router::new()
        .route("/fpl/player/:id", get(get_player))
        .route("/table", get(get_table))
        .nest_service("/", frontend)
        .with_state(state)
        .layer(CorsLayer::permissive());

    let port = app_config.server_port.unwrap_or(8000);
    let serve_addr = format!("0.0.0.0:{port}");
    // Run the server
    tracing::info!("Server running on http://{serve_addr}");
    let listener = tokio::net::TcpListener::bind(serve_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_player(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<String, (StatusCode, String)> {
    match state.endpoints.read() {
        Ok(ep) => {
            let full_name = propcomp::get_player_full_name(&*ep, id);
            Ok(format!("Player: {} with id {}\n", full_name, id))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Error reading endpoints"),
        )),
    }
}

async fn get_table(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.table.read() {
        Ok(t) => match serde_json::to_string(t.deref()) {
            Ok(json) => Ok((
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, "application/json")],
                json,
            )),
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to serialize league table"),
            )),
        },
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Error reading league table"),
        )),
    }
}

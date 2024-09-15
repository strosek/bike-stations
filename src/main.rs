mod handler;
mod model;
mod database;

use crate::handler::stations::{get_station_status, ingest_data};
use axum::routing::post;
use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};
use handler::stations::get_stations;

#[derive(Clone)]
struct AppState {
    db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    println!("Movatic Bike Stations API Backend Running ...");
    
    let state = AppState {
        db_pool: database::database_pool(5, "postgresql://root@localhost:26257/stations").await
    };
    
    let app = Router::new()
        .route("/stations", get(get_stations))
        .route("/stations/:station_id/status", get(get_station_status))
        .route("/ingest", post(ingest_data))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

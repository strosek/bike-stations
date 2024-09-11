mod handler;
mod model;

use crate::handler::stations::{get_station_status, ingest_data};
use axum::routing::post;
use axum::{routing::get, Router};
use handler::stations::get_stations;

#[tokio::main]
async fn main() {
    println!("Movatic Bike Stations API Backend Running ...");
    let app = Router::new()
        .route("/stations", get(get_stations))
        .route("/stations/:station_id/status", get(get_station_status))
        .route("/ingest", post(ingest_data));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

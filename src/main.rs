mod handler;
mod model;

use axum::{routing::get, Router};

use crate::handler::stations::get_station_status;
use handler::stations::get_stations;

#[tokio::main]
async fn main() {
    println!("Movatic Bike Stations API Backend Running ...");
    let app = Router::new()
        .route("/stations", get(get_stations))
        .route("/stations/:station_id/status", get(get_station_status));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

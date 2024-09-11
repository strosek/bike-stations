use crate::model::stations::{PagedStations, StationInformation, StationStatus};
use axum::{extract, Json};
use http::StatusCode;
use sqlx::postgres::PgPoolOptions;

pub async fn get_stations() -> Result<Json<PagedStations>, StatusCode> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect("postgresql://root@127.0.0.1:26257/stations")
        .await
        .unwrap();

    let stations: Vec<StationInformation> =
        sqlx::query_as("SELECT * FROM information LIMIT $1 OFFSET $2")
            .bind(10)
            .bind(0)
            .fetch_all(&pool)
            .await
            .unwrap();

    let paged_stations = PagedStations {
        stations,
        page: 0,
        total_pages: 1,
    };

    Ok(Json(paged_stations))
}

pub async fn get_station_status(
    extract::Path(station_id): extract::Path<String>,
) -> Result<Json<StationStatus>, StatusCode> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect("postgresql://root@127.0.0.1:26257/stations")
        .await
        .unwrap();

    let status: StationStatus =
        sqlx::query_as("SELECT * FROM status WHERE station_id = $1 LIMIT 1")
            .bind(station_id)
            .fetch_one(&pool)
            .await
            .unwrap();

    Ok(Json(status))
}

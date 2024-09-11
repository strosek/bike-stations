use crate::model::gbfs::Gbfs;
use crate::model::stations::{
    PagedStations, StationInformation, StationInformationList, StationStatus, StationStatusList,
};
use axum::{extract, Json};
use http::StatusCode;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, QueryBuilder};

fn connection_url() -> &'static str {
    "postgresql://root@127.0.0.1:26257/stations"
}

pub async fn get_stations() -> Result<Json<PagedStations>, StatusCode> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(connection_url())
        .await
        .unwrap();

    let stations: Vec<StationInformation> =
        sqlx::query_as("SELECT * FROM information LIMIT $1 OFFSET $2")
            .bind(1000)
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
        .connect(connection_url())
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

fn extract_feed_urls(gbfs_body: Gbfs) -> (String, String) {
    let name_urls = gbfs_body.data.unwrap().en.unwrap().feeds.unwrap();
    let mut info_url: Option<String> = None;
    let mut status_url: Option<String> = None;
    for name_url in name_urls {
        if let Some(name) = name_url.name {
            if name == "station_information" {
                info_url = Some(name_url.url.unwrap());
            } else if name == "station_status" {
                status_url = Some(name_url.url.unwrap());
            }
        }
    }

    (info_url.unwrap(), status_url.unwrap())
}

async fn ingest_station_info(pool: &Pool<Postgres>, info_url: String) -> Result<u64, StatusCode> {
    // Insert data for station_information
    let mut info_list: StationInformationList = Default::default();
    match reqwest::get(info_url.as_str()).await {
        Ok(resp) => {
            let text = resp.text().await.unwrap();
            info_list = serde_json::from_str(&text).unwrap();
        }
        Err(_) => {
            eprintln!("Could not read data from remote station_information JSON.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO information(id, name, address, latitude, longitude) ");

    let info_data = info_list.data.unwrap().stations.unwrap();
    query_builder.push_values(info_data.into_iter(), |mut b, info| {
        b.push_bind(info.station_id)
            .push_bind(info.name)
            .push_bind(info.address)
            .push_bind(info.lat)
            .push_bind(info.lon);
    });

    let query = query_builder.build();

    Ok(query.execute(pool).await.unwrap().rows_affected())
}

async fn ingest_station_status(pool: &Pool<Postgres>, url: String) -> Result<u64, StatusCode> {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO status(station_id, is_returning, is_renting, is_installed, num_docks_available, num_bikes_available, last_reported) ");

    let mut status_list: StationStatusList = Default::default();
    match reqwest::get(url.as_str()).await {
        Ok(resp) => {
            let text = resp.text().await.unwrap();
            status_list = serde_json::from_str(&text).unwrap();
        }
        Err(_) => {
            eprintln!("Could not read data from remote station_status JSON.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let status_data = status_list.data.unwrap().stations.unwrap();
    query_builder.push_values(status_data.into_iter(), |mut b, status| {
        b.push_bind(status.station_id)
            .push_bind(status.is_returning)
            .push_bind(status.is_renting)
            .push_bind(status.is_installed)
            .push_bind(status.num_docks_available)
            .push_bind(status.num_bikes_available)
            .push_bind(status.last_reported);
    });

    let query = query_builder.build();

    Ok(query.execute(pool).await.unwrap().rows_affected())
}

pub async fn ingest_data(Json(gbfs_body): Json<Gbfs>) -> Result<String, StatusCode> {
    let (info_url, status_url) = extract_feed_urls(gbfs_body);

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(connection_url())
        .await
        .unwrap();

    let station_info_rows = ingest_station_info(&pool, info_url).await.unwrap();
    let station_status_rows = ingest_station_status(&pool, status_url).await.unwrap();

    Ok(format!(
        "Inserted {} station_information rows, {} station_status rows",
        station_info_rows, station_status_rows
    ))
}

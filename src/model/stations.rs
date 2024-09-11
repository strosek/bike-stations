#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct StationInformation {
    pub id: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StationInformationJson {
    pub station_id: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct PagedStations {
    pub stations: Vec<StationInformation>,
    pub page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct StationStatus {
    pub station_id: String,
    pub is_returning: Option<bool>,
    pub is_renting: Option<bool>,
    pub is_installed: Option<bool>,
    pub num_docks_available: Option<i64>,
    pub num_bikes_available: Option<i64>,
    pub last_reported: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StationStatusJson {
    pub station_id: String,
    pub is_returning: Option<i64>,
    pub is_renting: Option<i64>,
    pub is_installed: Option<i64>,
    pub num_docks_available: Option<i64>,
    pub num_bikes_available: Option<i64>,
    pub last_reported: Option<i64>,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct StationStatusList {
    pub ttl: Option<i64>,
    pub last_updated: Option<i64>,
    pub data: Option<StationStatusData>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StationStatusData {
    pub stations: Option<Vec<StationStatusJson>>,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct StationInformationList {
    pub ttl: Option<i64>,
    pub last_updated: Option<i64>,
    pub data: Option<StationInformationData>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StationInformationData {
    pub stations: Option<Vec<StationInformationJson>>,
}

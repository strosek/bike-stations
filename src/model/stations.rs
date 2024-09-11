#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct StationInformation {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) latitude: Option<f64>,
    pub(crate) longitude: Option<f64>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StationInformationJson {
    pub(crate) station_id: String,
    pub(crate) name: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) lat: Option<f64>,
    pub(crate) lon: Option<f64>,
}

#[derive(Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct PagedStations {
    pub(crate) stations: Vec<StationInformation>,
    pub(crate) page: i64,
    pub(crate) total_pages: i64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct StationStatus {
    pub(crate) station_id: String,
    pub(crate) is_returning: Option<bool>,
    pub(crate) is_renting: Option<bool>,
    pub(crate) is_installed: Option<bool>,
    pub(crate) num_docks_available: Option<i64>,
    pub(crate) num_bikes_available: Option<i64>,
    pub(crate) last_reported: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StationStatusJson {
    pub(crate) station_id: String,
    pub(crate) is_returning: Option<i64>,
    pub(crate) is_renting: Option<i64>,
    pub(crate) is_installed: Option<i64>,
    pub(crate) num_docks_available: Option<i64>,
    pub(crate) num_bikes_available: Option<i64>,
    pub(crate) last_reported: Option<i64>,
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

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Gbfs {
    pub ttl: Option<i64>,
    pub data: Option<Data>,
    pub version: Option<String>,
    pub last_updated: Option<i64>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub en: Option<En>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct En {
    pub feeds: Option<Vec<NameUrl>>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NameUrl {
    pub name: Option<String>,
    pub url: Option<String>,
}

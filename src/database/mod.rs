use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn database_pool(max_connections: u32, url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await.unwrap() //.expect("Couldn't connect with the database")
}

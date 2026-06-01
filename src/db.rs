use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_db(database_url: &str) -> PgPool {
    // connect to db using connection pool
    PgPoolOptions::new()
        .max_connections(15)
        .connect(&database_url)
        .await
        .expect("database not connected")
}

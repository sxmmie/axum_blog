use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

use crate::{db::init_db, routemount::route::create_router};

mod db;
mod models;
mod routemount;
mod routes; // routes

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Declare env variable
    let database_url = std::env::var("DATABASE_URL").expect("database is missing in env");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:7870".to_string());

    // connect to db using connection pool
    let db_pool = init_db(&database_url).await; // init_db receives reference
    let app = create_router(db_pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    println!("server running in 7879");
    axum::serve(listener, app).await.unwrap();
}

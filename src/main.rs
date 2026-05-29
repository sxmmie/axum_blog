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
use sqlx::{PgPool, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Declare env variable
    let database_url = std::env::var("DATABASE_URL").expect("database is missing in env");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:7870".to_string());

    // connect to db using connection pool
    let db_connection_pool = PgPoolOptions::new()
        .max_connections(15)
        .connect(&database_url)
        .await
        .expect("database not connected");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello_post))
        .route("/user/{user_id}", get(path_extractor))
        .route("/query", get(query_extractor))
        .route("/json", post(request_body))
        .route("/players", get(get_players))
        .with_state(db_connection_pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    println!("server running in 7879");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    is_tall: bool,
}

// JSON body
async fn request_body(Json(payload): Json<User>) -> Json<User> {
    let rs = User {
        // name: = payload.name = "Ayo",
        name: String::from("Sammie"),
        age: 18,
        is_tall: true,
    };

    Json(rs)
}

#[derive(Serialize)]
struct PlayerRow {
    name: String,
    age: i32,
    wing: i32,
    player_id: i32,
}

async fn get_players(State(pg_connection_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(PlayerRow, r#"SELECT * FROM players ORDER BY player_id"#)
        .fetch_all(&pg_connection_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((StatusCode::OK, json!({"success": true, "data": rows}).to_string()))
}

// IntoResponseTrait converts any function into HTTP response in the client
async fn query_extractor(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    format!("query params is {:?}", params) // This {:?} is used when printing out collections (coes not implement the Display trait)
}

async fn path_extractor(Path(user_id): Path<u32>) -> String {
    // u32 means it can only be positve, no negative
    format!("the user id is {}", user_id)
}

async fn hello_post() -> &'static str {
    "Hello World"
}

use crate::{db::init_db, routemount::route::create_router};

mod db;
mod models;
mod routemount;
mod routes; // routes
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Declare env variable
    let database_url = std::env::var("DATABASE_URL").expect("database is missing in env");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:7870".to_string());

    // connect to db using connection pool
    let db_pool = init_db(&database_url).await; // init_db receives reference
    let app = create_router(db_pool);

    // Listeners - run our app with hyper, listening globally on port 7870
    let listener = tokio::net::TcpListener::bind(server_address.clone()).await.unwrap();
    println!("server running on {}", server_address);
    axum::serve(listener, app).await.unwrap();
}

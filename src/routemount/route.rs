use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::routes::player::get_players;

pub fn create_router(pool: PgPool) -> Router {
    Router::new().route("/players", get(get_players)).with_state(pool)
}

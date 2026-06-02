use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::routes::player::{create_player, get_players};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/players", get(get_players))
        .with_state(pool)
        .route("/palyers", post(create_player))
        .with_state(pool)
}

use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::PgPool;

use crate::models::player::CreatePlayerReq;

// pg == pg_connection_pool
pub async fn get_players(State(pg): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(PlayerRow, r#"SELECT * FROM players ORDER BY player_id"#)
        .fetch_all(&pg)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "message": e.to_string()}).to_string()))?;

    Ok((StatusCode::OK, json!({"success": true, "data": rows}).to_string()))
}

pub async fn create_player(State(pg): State<PgPool>, Json(player): Json<CreatePlayerReq>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        PlayerRow,
        "INSERT INTO players (name, age, wing) VALUES 
                                            ($1, $2, $3 RETURNING name, age, wing, player_id",
        player.name,
        player.age,
        player.wing
    )
    .fetch_one(&pg)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "message": e.to_string()}).to_string()))?;

    Ok((StatusCode::OK, json!({"success": true, "data": row}).to_string()))
}

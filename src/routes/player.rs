use axum::{extract::State, http::StatusCode};
use serde_json::json;
use sqlx::PgPool;

pub async fn get_players(
    State(pg_connection_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
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

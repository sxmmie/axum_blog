use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlayerRow {
    pub name: String,
    pub age: i32,
    pub wing: i32,
    pub player_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlayerReq {
    pub name: String,
    pub age: i32,
    pub wing: i32,
}

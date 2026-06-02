use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PlayerRow {
    pub name: String,
    pub age: i32,
    pub wing: i32,
    pub player_id: i32,
}

#[derive(Deserialize)]
pub struct CreatePlayerReq {
    pub name: String,
    pub age: i32,
    pub wing: i32,
}

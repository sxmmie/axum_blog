use serde::Serialize;

#[derive(Serialize)]
pub struct PlayerRow {
    pub name: String,
    pub age: i32,
    pub wing: i32,
    pub player_id: i32,
}

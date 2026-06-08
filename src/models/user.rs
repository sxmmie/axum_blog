use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub is_tall: bool,
}

pub struct LoginUser {}

pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

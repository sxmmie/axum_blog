use axum::http::StatusCode;
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub sub: String,
    pub exp: usize,
}

pub async fn verify_auth_token(TypedHeader(auth): TypedHeader<Authorization<Bearer>>) -> Result<Claims, StatusCode> {
    // extra raw token in a string
    let token = auth.token();

    // load secrets from env
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "my_secret".into());

    // attempt to decode and validate token
    let token_data = decode(token, &DecodingKey::from_secret(secret.as_bytes), &Validation::default()).map_err(|_| StatusCode::UNAUTHORIZED)
}

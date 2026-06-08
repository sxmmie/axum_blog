use axum::Json;
use axum::{extract::State, http::StatusCode};
use bcrypt::hash;
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::models::user::{LoginUser, RegisterUser};
use crate::utils::jwt::Claims;

pub async fn register_user(State(pg): State<PgPool>, Json(payload): Json<RegisterUser>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let hashed = hash(payload.password, 12).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email, password_hash"#,
        payload.name,
        payload.email,
        hashed
    )
    .fetch_optional(&pg)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| (StatusCode::CONFLICT, "Email already registered".to_string()))?;

    // Ok((StatusCode::CREATED, Json(user)));
    Ok(Json(user))
}

// Login Method
pub async fn login_user(State(pg): State<PgPool>, Json(payload): Json<LoginUser>) -> Result<Json<Value>, (StatusCode, String)> {
    let user_opt = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", payload.email)
        .fetch_optional(&pg)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid credentials".to_string()))?;

    let user = match user_opt {
        Some(u) => u,
        None => return Err((StatusCode::UNAUTHORIZED, "invalid credentials".into())),
    };

    // validate user provided password against password stored in the DB
    let valid = verify(&payload.password, &user.password_hash).map_err(|_| (StatusCode::UNAUTHORIZED, "invalid password".to_string()))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "invalid password".into()));
    }

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "mysecret".into()); // mysecret is the fallback
    let exp = Utc::now() + Duration::hours(1);

    // compose claims
    let claims = Claims {
        id: user.id,
        sub: user.email.clone(),
        exp: exp.timestamp() as usize,
    };

    // compose token
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({"token": token})))
}

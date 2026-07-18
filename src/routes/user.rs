use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use bcrypt::{hash, verify};
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::models::user::{LoginUser, RegisterUser, User};
use crate::utils::errorhandler::AppError;
use crate::utils::jwt::{Claims, verify_auth_token};

#[derive(Debug, Deserialize)]
pub struct UserQueryParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
}

pub async fn register_user(State(pg): State<PgPool>, Json(payload): Json<RegisterUser>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let hashed = hash(payload.password, 12).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email, password_hash"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&hashed)
    .fetch_optional(&pg)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| (StatusCode::CONFLICT, "Email already registered".to_string()))?;

    Ok((StatusCode::CREATED, json!({"success": true, "data": user}).to_string()))
}

// Login Method
pub async fn login_user(State(pg): State<PgPool>, Json(payload): Json<LoginUser>) -> Result<Json<Value>, AppError> {
    if payload.email.trim().is_empty() {
        return Err(AppError::bad_request("Email is required"));
    }

    if payload.password.trim().is_empty() {
        return Err(AppError::bad_request("Password is required"));
    }

    let user_opt = sqlx::query_as::<_, User>("SELECT id, name, email, password_hash FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&pg)
        .await
        .map_err(AppError::from)?;

    let user = match user_opt {
        Some(u) => u,
        None => return Err(AppError::not_found("user not found")),
    };

    // validate user provided password against password stored in the DB
    let valid = verify(&payload.password, &user.password_hash).map_err(|_| AppError::unauthorized("invalid credentials"))?;

    if !valid {
        return Err(AppError::unauthorized("invalid credentials"));
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
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).map_err(|_| AppError::Unexpected)?;

    Ok(Json(json!({"token": token})))
}

pub async fn protected_route(State(pg): State<PgPool>, TypedHeader(auth): TypedHeader<Authorization<Bearer>>) -> Result<Json<User>, StatusCode> {
    let claims = verify_auth_token(TypedHeader(auth)).await?;
    println!("{:?}", claims);

    let user = sqlx::query_as::<_, User>("SELECT id, name, email, password_hash FROM users WHERE email = $1")
        .bind(&claims.sub)
        .fetch_one(&pg)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(user))
}

pub async fn get_users(State(pg): State<PgPool>, Query(params): Query<UserQueryParams>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * limit;

    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash FROM users ORDER BY id DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&pg)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&pg)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = serde_json::json!({
        "page": page,
        "limit": limit,
        "total": total_count,
        "data": users
    });

    Ok(Json(response))
}

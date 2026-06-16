use axum::Json;
use axum::extract::Query;
use axum::{extract::State, http::StatusCode};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use bcrypt::{hash, verify};
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::{Value, json};
use sqlx::{PgPool, QueryBuilder, query_builder};

use crate::models::user::{LoginUser, RegisterUser, User};
use crate::utils::errorhandler::AppError;
use crate::utils::jwt::{Claims, verify_auth_token};

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
pub async fn login_user(State(pg): State<PgPool>, Json(payload): Json<LoginUser>) -> Result<Json<Value>, AppError> {
    if payload.email.trim().is_empty() {
        return Err(AppError::bad_request("Email is required"));
    }

    if payload.password.trim().is_empty() {
        return Err(AppError::bad_request("Password is required"));
    }

    let user_opt = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", payload.email)
        .fetch_optional(&pg)
        .await
        .map_err(|e| AppError::from(e))?;

    let user = match user_opt {
        Some(u) => u,
        None => return Err(AppError::not_found("user not found")),
    };

    // validate user provided password against password stored in the DB
    let valid = verify(&payload.password, &user.password_hash).map_err(|_| AppError::unauthorized("invalid credentials"))?;

    if !valid {
        return Err(AppError::unauthorized("invalid credentials"))?;
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

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", claims.sub)
        .fetch_one(&pg)
        .await
        .map_err(|_| AppError::unauthorized("you are not permitted to access this resource"))?;

    Ok(Json(user))
}

pub async fn get_users(State(pg): State<PgPool>, Query(params): Query<UserQueryParams>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // For every pagination, there is pageSize and Limit
    let page = params.page.unwrap_or(1);
    let limit = params () it.unwrap_or(10);
    let offset = (page -1) * limit;

    let mut query_builder = QueryBuilder::new("SELECT * FROM users WHERE 1=1");

    // name filter
    if let Some(name) = params.name{
        query_builder.push(" AND name ILIKE ");
        query_builder.push_bind(format!("%{}%", name))
    }

    // email filter
    if let Some(email) = params.email{
        query_builder.push(" AND name ILIKE ");
        query_builder.push_bind(format!("%{}%", email))
    }

    query_builde.push(" ORDER BY id DESC ");
    query_builde.push(" LIMIT ");
    query_builde.push_bind(limit);
    query_builde.push(" OFFSET ");
    query_builder.push_bind(offset);

    let query = query_builder.build_query_as()::<>(User);

    let users = query.fetch_all(executor);
}

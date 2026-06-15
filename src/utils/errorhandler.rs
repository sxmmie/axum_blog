use axum::{
    Json,                               // Used to return JSON responses
    http::StatusCode,                   // HTTP status enums (200, 400, 500, etc)
    response::{IntoResponse, Response}, // Trait + typee for converting errors into HTTP responses
};
use serde_json::json;
use thiserror::Error;

// Define a custom error type for the entire application
#[derive(Debug, Error)]
pub enum AppError {
    // Automatically convert sqlx::Error -> AppError::SqlxErrir using the #[from] attribute
    // Convert sqlx::Error automatically into this variant using #[from]
    // This lets you use "?" after SQL Queries
    #[error("Database query failed: {0}")]
    SqlxError(#[from] sqlx::Error),

    // Error for invalid client input (400)
    // #[error(Bad Request:) {0}]:
    // Defines the human-readable error message {0} inserts the inner value of the enum variant
    // Used when you call: error.to_string() or print the error
    #[error("Bad request: {0}")]
    BadRequest(String),

    // Error fir unauthorized requests (401)
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    // Error for unauthenticated requests (401)
    #[error("Forbidden: {0}")]
    Forbidden(String),

    // Validation error shortcut (also 400)
    #[error("Validation error: {0}")]
    ValidationError(String),

    // Generic fallback error
    #[error("Unexpected server error")]
    Unexpected,
}

impl AppError {
    pub fn bad_request<T: Into<String>>(msg: T) -> Self {
        AppError::BadRequest(msg.into())
    }

    // Create an unauthorized error
    // Self means "return the same type as the Impl back"
    // same as pub fn unauthorized<T: Into<String>>(msg: T) -> AppError
    pub fn unauthorized<T: Into<String>>(msg: T) -> Self {
        AppError::Unauthorized(msg.into())
    }

    // Create a forbidden error
    pub fn forbidden<T: Into<String>>(msg: T) -> Self {
        AppError::Forbidden(msg.into())
    }

    // Create validation error
    pub fn validation<T: Into<String>>(msg: T) -> Self {
        AppError::ValidationError(msg.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::SqlxError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),

            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),

            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),

            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),

            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),

            AppError::Unexpected => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "success": false,
            "error": {
                "message": message,
                "kind": format!("{:?}", self)
            }
        }));
        (status, body).into_response()
    }
}

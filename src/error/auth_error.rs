use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    NoAuthToken,
    TokenWrongFormat,
    TokenExpired,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        eprintln!("Error: {:?}", self);
        let (status, err_msg) = match self {
            AuthError::NoAuthToken => (StatusCode::UNAUTHORIZED, "No auth token"),
            AuthError::TokenWrongFormat => (StatusCode::UNAUTHORIZED, "Wrong token format"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}

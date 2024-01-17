use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use serde_json::json;

#[derive(Debug)]
pub enum ValidationError {
    ValidationErrorEmail,
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        eprintln!("Error: {:?}", self);
        let (status, err_msg) = match self {
            ValidationError::ValidationErrorEmail => (StatusCode::BAD_REQUEST, "Not an email"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}

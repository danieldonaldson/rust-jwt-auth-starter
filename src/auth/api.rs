use axum::Extension;
use axum::{extract::State, http::StatusCode, middleware, response::IntoResponse, Json, Router};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use tower::ServiceBuilder;
use validator::Validate;

use crate::utility::auth::create_jwt;
use crate::{auth::auth_config::AuthConfig, utility::auth::LoginPayload};
use crate::{error::validationation_error::ValidationError, mw};

use super::jwt_claims::JwtClaims;

pub fn routes(env_config: AuthConfig) -> Router {
    let protected_routes = Router::new()
        .route("/check_auth", axum::routing::get(check_auth))
        .route_layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            env_config.clone(),
            mw::mw_auth::mw_require_auth,
        )));

    Router::new()
        .route("/login", axum::routing::post(handler_login))
        .with_state(env_config.clone())
        .nest("/api/v1", protected_routes)
}

async fn check_auth(Extension(jwt): Extension<JwtClaims>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!(
            "Welcome {}! Your token expires on {}",
            jwt.email,
            convert_to_datetime(jwt.exp)
        ),
    )
}

async fn handler_login(
    State(config): State<AuthConfig>,
    Json(login_payload): Json<LoginPayload>,
) -> Result<Json<Value>, ValidationError> {
    let validation = login_payload.validate();
    if validation.is_err() {
        return Err(ValidationError::ValidationErrorEmail);
    }

    let email = login_payload.email.as_str();
    let (jwt, exp) = create_jwt(email, config.jwt_secret_key);

    Ok(Json(serde_json::json!({ "token": jwt,
                                "expires": exp, })))
}

fn convert_to_datetime(epoch: u64) -> String {
    let naive_datetime = NaiveDateTime::from_timestamp_opt(epoch as i64, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
    datetime.to_string()
}

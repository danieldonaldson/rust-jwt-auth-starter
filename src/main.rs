use auth::auth_config::AuthConfig;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod error;
mod mw;
mod utility;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT").expect("Port not found in .env");
    let address = std::env::var("ADDRESS").expect("Address not found in .env");
    let app = app().await.into_make_service();

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", address, port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn app() -> Router {
    let env_config = AuthConfig::new();
    let auth_routes = auth::api::routes(env_config);
    Router::new()
        .route("/health", get(health))
        .merge(auth_routes)
        .layer(CorsLayer::new().allow_origin(tower_http::cors::Any))
        .layer(TraceLayer::new_for_http())
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

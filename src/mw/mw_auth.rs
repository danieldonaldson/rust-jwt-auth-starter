use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::auth_config::AuthConfig;
use crate::auth::jwt_claims::JwtClaims;
use crate::error::auth_error::AuthError;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::{extract::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn mw_require_auth(
    State(config): State<AuthConfig>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // dbg!(&auth_token);
    let auth_token = headers.get("Bearer").ok_or(AuthError::NoAuthToken)?;
    let auth_token = auth_token.to_str().unwrap().to_owned();

    let (email, exp) = parse_token(auth_token, &config.jwt_secret_key)?;

    let jwt = JwtClaims { email, exp };
    req.extensions_mut().insert(jwt);

    Ok(next.run(req).await)
}

fn parse_token(token: String, secret: &String) -> Result<(String, u64), AuthError> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let token_data = decode::<JwtClaims>(&token, &decoding_key, &Validation::default());

    if let Ok(token) = token_data {
        let expiration_time = token.claims.exp;

        let current_time = get_current_time();

        if expiration_time < current_time {
            return Err(AuthError::TokenExpired);
        }

        // Retrieve the username from the JWT claims
        let username = token.claims.email;
        let exp = token.claims.exp;

        // Use the username as needed
        // println!("Username: {}", username);
        Ok((username, exp))
    } else {
        Err(AuthError::TokenWrongFormat)
    }
}

fn get_current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

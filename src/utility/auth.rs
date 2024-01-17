use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use validator::Validate;

use crate::auth::jwt_claims::JwtClaims;

pub fn create_jwt(email: &str, jwt_secret_key: String) -> (String, u64) {
    let claims = &JwtClaims::new(email);
    (
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(jwt_secret_key.as_ref()),
        )
        .unwrap(),
        claims.exp,
    )
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,
}

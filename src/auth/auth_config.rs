#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret_key: String,
}

impl AuthConfig {
    pub fn new() -> Self {
        let jwt_secret_key = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not set");
        AuthConfig { jwt_secret_key }
    }
}

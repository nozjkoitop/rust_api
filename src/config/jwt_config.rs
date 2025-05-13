use serde::Deserialize;
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize, Clone)]
pub struct AuthConfig {
    /// HMAC secret for signing tokens
    pub jwt_secret: String,
    /// Token TTL in seconds
    pub token_ttl: i64,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        AuthConfig {
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            token_ttl: env::var("TOKEN_TTL")
                .unwrap_or_else(|_| "3600".into())
                .parse()
                .expect("TOKEN_TTL must be an integer"),
        }
    }
}

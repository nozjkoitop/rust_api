use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::config::jwt_config::AuthConfig;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JwtManager {
    encoding: EncodingKey,
    decoding: DecodingKey,
    ttl: usize,
}

impl JwtManager {
    pub fn new(cfg: &AuthConfig) -> Self {
        JwtManager {
            encoding: EncodingKey::from_secret(cfg.jwt_secret.as_ref()),
            decoding: DecodingKey::from_secret(cfg.jwt_secret.as_ref()),
            ttl: cfg.token_ttl as usize,
        }
    }

    pub fn create_token(&self, sub: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: sub.to_owned(),
            exp: now + self.ttl,
        };
        encode(&Header::default(), &claims, &self.encoding)
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(token, &self.decoding, &Validation::default())
    }
}

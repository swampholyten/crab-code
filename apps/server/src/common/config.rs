use crate::common::error::Result;
use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,

    pub service_host: String,
    pub service_port: String,

    pub jwt_secret: String,
    pub jwt_keys: JwtKeys,
    pub jwt_expire_access_token_seconds: i64,
    pub jwt_expire_refresh_token_seconds: i64,
    pub jwt_validation_leeway_seconds: i64,
    pub jwt_enable_revoked_tokens: bool,
}

#[derive(Clone)]
pub struct JwtKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl JwtKeys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl std::fmt::Debug for JwtKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtKeys").finish()
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let jwt_secret = env_get("JWT_SECRET");

        let config = Config {
            database_url: env_get("DATABASE_URL"),
            service_host: env_get("SERVICE_HOST"),
            service_port: env_parse("SERVICE_PORT"),
            jwt_keys: JwtKeys::new(jwt_secret.as_bytes()),
            jwt_secret,
            jwt_expire_access_token_seconds: env_parse("JWT_EXPIRE_ACCESS_TOKEN_SECONDS"),
            jwt_expire_refresh_token_seconds: env_parse("JWT_EXPIRE_REFRESH_TOKEN_SECONDS"),
            jwt_validation_leeway_seconds: env_parse("JWT_VALIDATION_LEEWAY_SECONDS"),
            jwt_enable_revoked_tokens: env_parse("JWT_ENABLE_REVOKED_TOKENS"),
        };

        Ok(config)
    }
}

#[inline]
fn env_get(key: &str) -> String {
    match std::env::var(key) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("{} {}", key, e);
            tracing::error!(msg);
            panic!("{msg}");
        }
    }
}

#[inline]
fn env_parse<T: std::str::FromStr>(key: &str) -> T {
    env_get(key).parse().map_or_else(
        |_| {
            let msg = format!("Failed to parse: {}", key);
            tracing::error!(msg);
            panic!("{msg}");
        },
        |v| v,
    )
}

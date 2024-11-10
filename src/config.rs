use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
struct EnvConfig {
    host_name: String,
    host_port: Option<u16>,
    db_host: String,
    db_namespace: String,
    db_database: String,
    db_user: String,
    db_pswd: String,
    db_version: Option<usize>,
    jwt_secret: String,
    jwt_cookie_name: Option<String>,
}

pub(crate) struct Config {
    pub(crate) host_name: String,
    pub(crate) host_port: Option<u16>,
    pub(crate) db_host: String,
    pub(crate) db_namespace: String,
    pub(crate) db_database: String,
    pub(crate) db_user: String,
    pub(crate) db_pswd: String,
    pub(crate) db_version: Option<usize>,
    pub(crate) jwt_decode: DecodingKey,
    pub(crate) jwt_encode: EncodingKey,
    pub(crate) jwt_cookie_name: String,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Parse Error: {0}")]
    Parse(String),
    #[error("Missing Error: {0}")]
    Missing(String),
}

pub fn load_config() -> Result<Config, ConfigError> {
    let config = EnvConfig {
        host_name: std::env::var("HOST_NAME")
            .map_err(|_| ConfigError::Missing("Missing Env: `DB_PSWD`".to_string()))?,
        host_port: std::env::var("HOST_PORT")
            .ok()
            .map(|host_port| {
                u16::from_str(host_port.as_str())
                    .map_err(|_| ConfigError::Parse("Parse Failed: `DB_VERSION`".to_string()))
            })
            .transpose()?,
        db_host: std::env::var("DB_HOST")
            .map_err(|_| ConfigError::Missing("Missing Env: `DB_HOST`".to_string()))?,
        db_namespace: std::env::var("DB_NAMESPACE")
            .map_err(|_| ConfigError::Missing("Missing Env: `DB_NAMESPACE`".to_string()))?,
        db_database: std::env::var("DB_DATABASE")
            .map_err(|_| ConfigError::Missing("Missing Env: `DB_DATABASE`".to_string()))?,
        db_user: std::env::var("DB_USER")
            .map_err(|_| ConfigError::Missing("Missing Env: `DB_USER`".to_string()))?,
        db_pswd: std::env::var("DB_PSWD")
            .map_err(|_| ConfigError::Missing("Missing Env: `DB_PSWD`".to_string()))?,
        db_version: std::env::var("DB_VERSION")
            .ok()
            .map(|version| {
                usize::from_str(version.as_str())
                    .map_err(|_| ConfigError::Parse("Failed to parse `DB_VERSION`".to_string()))
            })
            .transpose()?,
        jwt_secret: std::env::var("JWT_SECRET")
            .map_err(|_| ConfigError::Missing("Missing: `JWT_SECRET`".to_string()))?,
        jwt_cookie_name: std::env::var("JWT_COOKIE_NAME").ok(),
    };

    Ok(Config {
        host_name: config.host_name,
        host_port: config.host_port,
        db_host: config.db_host,
        db_namespace: config.db_namespace,
        db_database: config.db_database,
        db_user: config.db_user,
        db_pswd: config.db_pswd,
        db_version: config.db_version,
        jwt_decode: DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        jwt_encode: EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        jwt_cookie_name: config.jwt_cookie_name.unwrap_or("session".to_string()),
    })
}

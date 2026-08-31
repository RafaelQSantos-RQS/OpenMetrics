use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "openmetrics", about = "Linux system monitoring dashboard")]
pub struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub metrics: MetricsConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsConfig {
    #[serde(default = "default_polling_interval")]
    pub polling_interval_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    #[serde(default = "default_db_path")]
    pub db_path: String,
    #[allow(dead_code)]
    #[serde(default = "default_retention_days")]
    pub retention_days: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password_hash")]
    pub password_hash: String,
    #[serde(default = "default_session_secret")]
    pub session_secret: String,
    #[serde(default = "default_session_expiry")]
    pub session_expiry_hours: u64,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_polling_interval() -> u64 {
    5
}

fn default_db_path() -> String {
    "data/metrics.db".to_string()
}

fn default_retention_days() -> u64 {
    30
}

fn default_username() -> String {
    "admin".to_string()
}

fn default_password_hash() -> String {
    // Default password: "admin" - CHANGE IN PRODUCTION
    "$2b$12$LJ3m4ys3Lz0wqV9rK5eQxuQHqFqVqVqVqVqVqVqVqVqVqVqVqVqVq".to_string()
}

fn default_session_secret() -> String {
    "change-me-to-a-random-secret-key".to_string()
}

fn default_session_expiry() -> u64 {
    24
}

impl Config {
    pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                host: default_host(),
                port: default_port(),
            },
            metrics: MetricsConfig {
                polling_interval_secs: default_polling_interval(),
            },
            database: DatabaseConfig {
                db_path: default_db_path(),
                retention_days: default_retention_days(),
            },
            auth: AuthConfig {
                username: default_username(),
                password_hash: default_password_hash(),
                session_secret: default_session_secret(),
                session_expiry_hours: default_session_expiry(),
            },
        }
    }
}

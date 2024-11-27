use anyhow::Context;
use std::env;

const SERVER_PORT_KEY: &str = "SERVER_PORT";
const DB_CONNECTION_KEY: &str = "DB_URL";

/// [Config] contains the necessary application config to run the application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: String,
    pub db_url: String,
}

impl Config {
    /// Creates a Config from the process environment.
    pub fn from_env() -> anyhow::Result<Self> {
        let server_port = load_env(SERVER_PORT_KEY)?;
        let db_url = load_env(DB_CONNECTION_KEY)?;

        Ok(Self {
            server_port,
            db_url,
        })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {key}"))
}

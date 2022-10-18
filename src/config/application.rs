use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub is_bind: bool,
    pub(crate) server: ServerConfig,
    pub(crate) logger: LoggerConfig,
    pub(crate) database: DatabaseConfig,
}

impl ApplicationConfig {
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) host: Option<String>,
}

#[derive(Deserialize)]
pub struct LoggerConfig {
    pub(crate) level: Option<String>,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub(crate) path: Option<PathBuf>,
}

use config::{builder::AsyncState, ConfigBuilder, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub is_bind: bool,
    pub port: u16,
    pub(self) host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PxollyConfig {
    pub(self) secret_key: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VKConfig {
    pub version: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub pxolly: PxollyConfig,
    pub vk: VKConfig,
}

impl ServerConfig {
    pub fn host(&self) -> Option<String> {
        if !self.host.is_empty() {
            return Some(self.host.to_string());
        };
        None
    }
}

impl PxollyConfig {
    pub fn secret_key(&self) -> String {
        self.secret_key.clone()
    }
}

impl AppConfig {
    pub async fn new() -> Result<Self, ConfigError> {
        ConfigBuilder::<AsyncState>::default()
            .add_source(File::new("config/config.toml", FileFormat::Toml))
            .build()
            .await?
            .try_deserialize()
    }
}

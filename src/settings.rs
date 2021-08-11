use anyhow::Result;
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub auto_connect: bool,
    pub port: u16,
    pub secret_key: String,
    pub pxolly_token: String,
    pub access_token: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::default();

        config.merge(File::with_name("conf/settings.toml"))?;
        return config.try_into();
    }
}

impl AsRef<Settings> for Settings {
    fn as_ref(&self) -> &Settings {
        &self
    }
}

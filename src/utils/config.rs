use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PxollyConfig {
    pub auto_connect: bool,
    pub port: u16,
    pub secret_key: String,
    pub pxolly_token: String,
    pub access_token: String,
    host: String,
}

impl PxollyConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::default();

        config.merge(File::with_name("conf/config.toml"))?;
        config.try_into()
    }

    pub fn host(&self) -> Option<String> {
        if !self.host.is_empty() {
            return Some(self.host.to_string())
        };
        None
    }
}

impl AsRef<PxollyConfig> for PxollyConfig {
    fn as_ref(&self) -> &PxollyConfig {
        self
    }
}

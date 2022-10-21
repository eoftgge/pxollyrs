pub mod application;
pub mod pxolly;
pub mod vk;

use crate::config::application::ApplicationConfig;
use crate::config::pxolly::PxollyConfig;
use crate::config::vk::VKConfig;
use config::{builder::AsyncState, ConfigBuilder, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct WebhookConfig {
    application: ApplicationConfig,
    vk: VKConfig,
    pxolly: PxollyConfig,
}

impl WebhookConfig {
    pub fn application(&self) -> &ApplicationConfig {
        &self.application
    }

    pub fn vk(&self) -> &VKConfig {
        &self.vk
    }

    pub fn pxolly(&self) -> &PxollyConfig {
        &self.pxolly
    }

    pub async fn new() -> Result<Self, ConfigError> {
        ConfigBuilder::<AsyncState>::default()
            .add_source(File::new("config.toml", FileFormat::Toml))
            .build()
            .await?
            .try_deserialize()
    }
}

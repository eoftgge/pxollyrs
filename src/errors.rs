use crate::pxolly::errors::PxollyError;
use crate::vkontakte::errors::VKontakteError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebhookError {
    #[error("Config: {0}")]
    Config(#[from] config::ConfigError),
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("VK: {0}")]
    VKontakte(#[from] VKontakteError),
    #[error("@pxolly: {0}")]
    Pxolly(#[from] PxollyError),
    #[error("Message: {0}")]
    Message(String),
}

impl From<&str> for WebhookError {
    fn from(text: &str) -> Self {
        Self::Message(text.into())
    }
}

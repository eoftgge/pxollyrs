use crate::vkontakte::responses::VKAPIError;
use thiserror::Error;
use crate::pxolly::errors::PxollyError;

#[derive(Debug, Error)]
pub enum WebhookError {
    #[error("Error in config: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Error in IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("VKAPI({}) - {}", .0.error_code, .0.error_msg)]
    VKAPI(VKAPIError),
    Pxolly(#[from] PxollyError),
    #[error("{0}")]
    Message(String),
}

impl From<&str> for WebhookError {
    fn from(text: &str) -> Self {
        Self::Message(text.into())
    }
}
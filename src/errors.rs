use crate::pxolly::api::responses::PxollyAPIError;
use crate::pxolly::types::responses::PxollyResponse;
use crate::vk::responses::VKAPIError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebhookError {
    #[error("Error in config: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Error in IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("Error in serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Error in reqwest: {0}")]
    HTTP(#[from] reqwest::Error),
    #[error("VKAPI({}) - {}", .0.error_code, .0.error_msg)]
    VKAPI(VKAPIError),
    #[error("Returning error code")]
    PxollyResponse(PxollyResponse),
    #[error("PxollyError({}) - {}", .0.error_code, .0.error_text)]
    PxollyAPI(PxollyAPIError),
    #[error("{0}")]
    Message(String),
}

impl From<&str> for WebhookError {
    fn from(text: &str) -> Self {
        Self::Message(text.into())
    }
}

pub type WebhookResult<T> = Result<T, WebhookError>;

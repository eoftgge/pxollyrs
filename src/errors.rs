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
    #[error("Error in json ser/de: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Error in reqwest: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Error in msgpack encode: {0}")]
    RmpEncoded(#[from] rmp_serde::encode::Error),
    #[error("Error in msgpack decode: {0}")]
    RmpDecoded(#[from] rmp_serde::decode::Error),
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

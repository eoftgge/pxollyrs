use crate::api::responses::APIResponseError;
use crate::pxolly::types::responses::PxollyResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PxollyError {
    #[error("Error in config: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Error in IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("Error in serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Error in reqwest: {0}")]
    HTTP(#[from] reqwest::Error),
    #[error("APIError({}) - {}", .0.error_code, .0.error_msg)]
    API(APIResponseError),
    #[error("Returning error code")]
    Response(PxollyResponse),
    #[error("{0}")]
    Message(String),
}

impl From<&str> for PxollyError {
    fn from(text: &str) -> Self {
        Self::Message(text.into())
    }
}

pub type PxollyResult<T> = Result<T, PxollyError>;

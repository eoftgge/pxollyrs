use crate::utils::models::PxollyResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PxollyError {
    #[error("Error during read/write to file.")]
    IO(#[from] std::io::Error),
    #[error("Error during de/serialization.")]
    Serde(#[from] serde_json::Error),
    #[error("Error http.")]
    HTTP(#[from] reqwest::Error),
    #[error("Error during send request to API.")]
    API(crate::api::response::APIError),
    #[error("Response.")]
    Response(PxollyResponse),
    #[error("Other's error.")]
    Other(String),
}

impl From<&str> for PxollyError {
    fn from(text: &str) -> Self {
        Self::Other(text.into())
    }
}

pub type PxollyResult<T> = Result<T, PxollyError>;

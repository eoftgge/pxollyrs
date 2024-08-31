use thiserror::Error;
use crate::pxolly::types::responses::api::PxollyAPIError;

#[derive(Debug, Error)]
pub enum PxollyError {
    #[error("HTTP: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API: {0:?}")]
    API(PxollyAPIError),
    #[error("Encoded: {0}")]
    RmpEncoded(#[from] rmp_serde::encode::Error),
    #[error("Decoded: {0}")]
    RmpDecoded(#[from] rmp_serde::decode::Error),
}
use crate::pxolly::types::responses::api::PxollyAPIError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PxollyError {
    #[error("HTTP: {0}")]
    HTTP(#[from] reqwest::Error),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API: {0:?}")]
    API(PxollyAPIError),
    #[error("Encoded: {0}")]
    RmpEncoded(#[from] rmp_serde::encode::Error),
    #[error("Decoded: {0}")]
    RmpDecoded(#[from] rmp_serde::decode::Error),
    #[error("QS: {0}")]
    QS(#[from] serde_qs::Error),
}

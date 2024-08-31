use thiserror::Error;
use crate::pxolly::types::responses::errors::PxollyResponseError;

#[derive(Debug, Error)]
pub enum PxollyError {
    Http(#[from] reqwest::Error),
    Json(#[from] serde_json::Error),
    API(#[from] PxollyResponseError),
    RmpEncoded(#[from] rmp_serde::encode::Error),
    RmpDecoded(#[from] rmp_serde::decode::Error),
}
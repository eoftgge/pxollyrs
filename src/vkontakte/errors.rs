use crate::vkontakte::types::responses::api::VKontakteAPIError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VKontakteError {
    #[error("HTTP: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API: {0:?}")]
    API(VKontakteAPIError),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
}

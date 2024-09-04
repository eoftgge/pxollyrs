use crate::vkontakte::types::responses::api::VKontakteAPIError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VKontakteError {
    #[error("HTTP: {0}")]
    HTTP(#[from] reqwest::Error),
    #[error("API: {0:?}")]
    API(VKontakteAPIError),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("QS: {0}")]
    QS(#[from] serde_qs::Error),
}

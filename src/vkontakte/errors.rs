use thiserror::Error;
use crate::vkontakte::types::responses::api::VKontakteAPIError;

#[derive(Debug, Error)]
pub enum VKontakteError {
    Http(#[from] reqwest::Error), 
    API(#[from] VKontakteAPIError),
    Json(#[from] serde_json::Error),
}
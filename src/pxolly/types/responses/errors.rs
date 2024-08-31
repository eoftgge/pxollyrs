use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PxollyWebhookError {
    #[serde(rename = "type")]
    pub error_type: PxollyErrorType,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PxollyErrorType {
    UnknownError,
    InternalServerError,
    ChatNotFound,
    UnknownEvent,
    #[serde(rename = "VK_API_ERROR")]
    VKontakteAPIError,
    AccessDenied,
    BotAccessDenied,
    NotInFriends,
    InvalidPrivacySettingsForInvite,
    #[serde(rename = "VK_LIMITS_REACHED")]
    VKontakteLimitsReached,
}

impl IntoResponse for PxollyErrorType {
    fn into_response(self) -> Response {
        let json = Json(self);
        json.into_response()
    }
}
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
    BotAccessDenied,
    NotInFriends,
    InvalidPrivacySettingsForInvite,
    #[serde(rename = "VK_LIMITS_REACHED")]
    VKontakteLimitsReached,
}
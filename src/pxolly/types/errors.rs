use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PxollyResponseError {
    #[serde(rename = "type")]
    error_type: PxollyErrorType,
    message: String,
}

#[derive(Deserialize, Serialize)]
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
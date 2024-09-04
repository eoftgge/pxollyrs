use crate::vkontakte::errors::VKontakteError;
use serde::{Deserialize, Serialize};
use crate::vkontakte::types::responses::VKontakteAPIError;

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

impl PxollyWebhookError {
    pub fn chat_not_found() -> Self {
        Self {
            message: None,
            error_type: PxollyErrorType::ChatNotFound,
        }
    }

    pub fn internal_server() -> Self {
        Self {
            message: None,
            error_type: PxollyErrorType::InternalServerError,
        }
    }
}

impl From<reqwest::Error> for PxollyWebhookError {
    fn from(_: reqwest::Error) -> Self {
        Self {
            message: Some("Error in HTTP".into()),
            error_type: PxollyErrorType::InternalServerError,
        }
    }
}

impl From<VKontakteError> for PxollyWebhookError {
    fn from(err: VKontakteError) -> Self {
        match err {
            VKontakteError::API(VKontakteAPIError { error_code: 6, .. }) => Self {
                error_type: PxollyErrorType::VKontakteLimitsReached,
                message: None,
            },
            VKontakteError::API(VKontakteAPIError { error_code: 9, .. }) => Self {
                error_type: PxollyErrorType::VKontakteLimitsReached,
                message: None,
            },
            VKontakteError::API(_) => Self {
                error_type: PxollyErrorType::VKontakteAPIError,
                message: None,
            },
            _ => Self {
                error_type: PxollyErrorType::InternalServerError,
                message: None,
            }
        }
    }
}

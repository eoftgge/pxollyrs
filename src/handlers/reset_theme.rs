use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::event_type::EventType;
use crate::pxolly::types::responses::errors::{PxollyErrorType, PxollyWebhookError};
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::messages::reset_conversation_style::ResetConversationStyleParams;
use crate::vkontakte::types::responses::VKontakteAPIError;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ResetThemeObject {
    chat_id: String,
    chat_local_id: Option<u64>,
}

pub struct ResetTheme {
    pub(crate) vkontakte: VKontakteAPI,
}

impl Handler for ResetTheme {
    const EVENT_TYPE: EventType = EventType::ResetTheme;
    type EventObject = ResetThemeObject;

    async fn handle(
        &self,
        object: Self::EventObject,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let peer_id = object
            .chat_local_id
            .ok_or_else(PxollyWebhookError::chat_not_found)?
            + 2_000_000_000;
        match self
            .vkontakte
            .messages()
            .reset_conversation_style(ResetConversationStyleParams {
                peer_id: peer_id as i64,
            })
            .await
        {
            Ok(_) => Ok(PxollyWebhookResponse::new(true)),
            Err(VKontakteError::API(VKontakteAPIError {
                error_code: 966, ..
            })) => Err(PxollyWebhookError {
                message: None,
                error_type: PxollyErrorType::VKontakteAPIError,
            }),
            Err(VKontakteError::API(VKontakteAPIError { error_code: 3, .. })) => {
                Err(PxollyWebhookError {
                    message: Some("Возможно, вебхук не имеет доступа к этому методу.".into()),
                    error_type: PxollyErrorType::BotAccessDenied,
                })
            }
            _ => Err(PxollyWebhookError {
                message: None,
                error_type: PxollyErrorType::VKontakteAPIError,
            }),
        }
    }
}

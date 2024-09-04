use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::event_type::EventType;
use crate::pxolly::types::responses::errors::{PxollyErrorType, PxollyWebhookError};
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::messages::set_conversation_style::SetConversationStyleParams;
use crate::vkontakte::types::responses::VKontakteAPIError;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SetThemeObject {
    chat_id: String,
    chat_local_id: Option<u64>,
    style: String,
}

pub struct SetTheme {
    pub(crate) vkontakte: VKontakteAPI,
}

impl Handler for SetTheme {
    const EVENT_TYPE: EventType = EventType::SetTheme;
    type EventObject = SetThemeObject;

    async fn handle(
        &self,
        object: Self::EventObject,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let params = SetConversationStyleParams {
            peer_id: (object
                .chat_local_id
                .ok_or_else(PxollyWebhookError::chat_not_found)?
                + 2_000_000_000) as i64,
            style: object.style,
        };
        match self
            .vkontakte
            .messages()
            .set_conversation_style(params)
            .await
        {
            Ok(_) => Ok(PxollyWebhookResponse::new(true)),
            Err(VKontakteError::API(VKontakteAPIError {
                error_code: 966, ..
            })) => Err(PxollyWebhookError {
                message: None,
                error_type: PxollyErrorType::BotAccessDenied,
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

use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::{PxollyErrorType, PxollyWebhookError};
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::messages::set_conversation_style::SetConversationStyleParams;
use crate::vkontakte::types::responses::VKontakteAPIError;

pub struct SetTheme {
    pub(crate) vkontakte: VKontakteAPI,
}

impl Handler for SetTheme {
    const EVENT_TYPE: &'static str = "set_theme";

    async fn handle(
        &self,
        event: PxollyEvent,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let params = SetConversationStyleParams {
            peer_id: (event.object.chat_local_id.unwrap() + 2_000_000_000) as i64,
            style: event.object.style.unwrap(),
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
            _ => Err(PxollyWebhookError {
                message: None,
                error_type: PxollyErrorType::VKontakteAPIError,
            }),
        }
    }
}

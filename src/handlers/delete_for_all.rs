use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::event_type::EventType;
use crate::pxolly::types::responses::errors::{PxollyErrorType, PxollyWebhookError};
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::messages::delete::MessagesDeleteParams;
use serde::Deserialize;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::responses::VKontakteAPIError;

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteForAllObject {
    chat_id: String,
    chat_local_id: Option<u64>,
    conversation_message_ids: Vec<u64>,
}

pub struct DeleteForAll {
    pub(crate) vkontakte: VKontakteAPI,
}

impl Handler for DeleteForAll {
    const EVENT_TYPE: EventType = EventType::DeleteForAll;
    type EventObject = DeleteForAllObject;

    async fn handle(
        &self,
        object: DeleteForAllObject,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let chat_id = object
            .chat_local_id
            .ok_or_else(PxollyWebhookError::chat_not_found)?;
        let params = MessagesDeleteParams {
            peer_id: (chat_id + 2_000_000_000) as i64,
            delete_for_all: 1,
            cmids: object.conversation_message_ids,
        };
        let response = self.vkontakte.messages().delete(params).await;
        let response = match response {
            Ok(res) => res,
            Err(VKontakteError::API(VKontakteAPIError { error_code: 924, .. })) => {
                return Err(PxollyWebhookError {
                    message: Some("не удалось удалить сообщения".into()),
                    error_type: PxollyErrorType::BotAccessDenied,
                })
            }
            Err(err) => {
                log::error!("e: {:?}", err);
                return Err(PxollyWebhookError {
                    message: Some("неизвестная ошибка...".into()),
                    error_type: PxollyErrorType::VKontakteAPIError,
                })
            }
        };

        let mut cmids = Vec::new();
        for (id, is_success) in response.iter() {
            if *is_success != 0 {
                cmids.push(*id);
            }
        }

        Ok(PxollyWebhookResponse::new(true).conversation_message_ids(cmids))
    }
}

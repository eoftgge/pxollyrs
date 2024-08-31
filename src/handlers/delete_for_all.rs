use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::messages::delete::MessagesDeleteParams;

pub struct DeleteForAll {
    pub(crate) vkontakte: VKontakteAPI,
}

impl Handler for DeleteForAll {
    const EVENT_TYPE: &'static str = "delete_for_all";

    async fn handle(&self, event: PxollyEvent) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let params = MessagesDeleteParams {
            peer_id: (event.object.chat_local_id.unwrap() + 2_000_000_000) as i64,
            delete_for_all: 1,
            cmids: event.object.conversation_message_ids
                .expect("Expect field: cmids")
        };
        let response = self.vkontakte.messages().delete(params).await?;

        let mut cmids = Vec::new();
        for (id, is_success) in response.iter() {
            if *is_success != 0 {
                cmids.push(*id);
            }
        }

        Ok(PxollyWebhookResponse::new(true).conversation_message_ids(cmids))
    }
}

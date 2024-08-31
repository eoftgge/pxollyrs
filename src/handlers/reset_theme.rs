use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::messages::reset_conversation_style::ResetConversationStyleParams;

pub struct ResetTheme {
    pub(crate) vkontakte: VKontakteAPI,
}

impl Handler for ResetTheme {
    const EVENT_TYPE: &'static str = "reset_theme";

    async fn handle(
        &self,
        event: PxollyEvent,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let peer_id = event.object.chat_local_id.unwrap() + 2_000_000_000;
        self.vkontakte
            .messages()
            .reset_conversation_style(ResetConversationStyleParams {
                peer_id: peer_id as i64,
            })
            .await?;
        Ok(PxollyWebhookResponse::new(true))
    }
}

use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::execute::ExecuteParams;

pub struct Sync {
    pub(crate) vkontakte: VKontakteAPI,
}

impl Handler for Sync {
    const EVENT_TYPE: &'static str = "sync";

    async fn handle(
        &self,
        event: PxollyEvent,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let message = event
            .object
            .message
            .as_ref()
            .expect("Expect field: messages");
        let params = serde_json::json!({
            "conversation_message_id": message.conversation_message_id,
            "text": message.text,
            "date": message.date,
            "from_id": message.from_id
        });
        let peer_id = self
            .vkontakte
            .execute::<i64>(ExecuteParams {
                code: EXECUTE_SYNC_CODE.into(),
                extras: params,
            })
            .await?;

        Ok(PxollyWebhookResponse::new(true).local_id(peer_id as u64))
    }
}

const EXECUTE_SYNC_CODE: &str = r#"
var h = API.messages.search({
    q: Args.text,
    count: 5
}).items;

var i = 0;
while(i < h.length) {
    if(Args.text == h[i].text && Args.from_id == h[i].from_id && Args.date == h[i].date && Args.conversation_message_id == h[i].conversation_message_id) {
        return h[i].peer_id;
    }
    i = i+1;
}
return false;
"#;

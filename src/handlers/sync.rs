
use crate::handlers::prelude::*;

pub struct Sync {
    pub(crate) vk_client: VKClient,
}

impl Handler for Sync {
    const EVENT_TYPE: &'static str = "sync";

    async fn handle(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let message = ctx.object.message.as_ref().expect("Expect field: messages");
        let params = serde_json::json!({
            "code": EXECUTE_SYNC_CODE,
            "conversation_message_id": message.conversation_message_id,
            "text": message.text,
            "date": message.date,
            "from_id": message.from_id
        });
        let chat_id = ctx.object.chat_id.as_ref().expect("Expect field: chat_id");

        if DatabaseChatModel::contains(chat_id, &ctx.database()).await? {
            return Ok(PxollyResponse::ErrorCode(5));
        }

        let peer_id = self.vk_client.api_request::<i64>("execute", params).await?;

        DatabaseChatModel {
            chat_uid: peer_id,
            chat_id: chat_id.into(),
        }
        .insert(&ctx.database())
        .await?;

        Ok(PxollyResponse::ConfirmationCode(
            ctx.object
                .success
                .as_ref()
                .expect("Expect field: success")
                .into(),
        ))
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

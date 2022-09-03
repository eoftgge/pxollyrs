use crate::errors::PxollyError;
use crate::handlers::HandlerContext;
use crate::utils::models::PxollyResponse;
use crate::{par, PxollyResult};

const SYNC_CODE: &str = r#"
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

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let message = ctx.object.message.as_ref().expect("Expect field: message");
    let params = par! {
        "code": SYNC_CODE,
        "conversation_message_id": message.conversation_message_id,
        "text": message.text,
        "date": message.date,
        "from_id": message.from_id
    };
    let chat_id = ctx.object.chat_id.as_ref().expect("Expect field: chat_id");

    if ctx.database.contains(chat_id).await {
        return Ok(PxollyResponse::ErrorCode(5));
    }

    let peer_id = ctx.client.api_request::<i64>("execute", params).await?;

    ctx.database
        .insert(chat_id.as_str(), peer_id as u64)
        .await
        .map_err(|_| PxollyError::Response(PxollyResponse::ErrorCode(3)))?;

    Ok(PxollyResponse::ConfirmationCode(
        ctx.object
            .success
            .as_ref()
            .expect("Expect field: success")
            .into(),
    ))
}

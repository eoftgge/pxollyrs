use crate::errors::PxollyError;
use crate::handlers::HandlerContext;
use crate::utils::models::PxollyResponse;
use crate::{par, PxollyResult};

const INVITE_CODE: &str = r#"
if(API.friends.areFriends({user_ids:Args.member_id})[0].friend_status==3) {
    return API.messages.addChatUser({
        chat_id: Args.chat_id,
        user_id: Args.member_id,
        visible_messages_count: Args.visible_messages_count
    });
}
return -100;
"#;

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let params = par! {
        "visible_messages_count": ctx.object.visible_messages_count.unwrap_or(0),
        "member_id": ctx.object.user_id.expect("Expect field: user_id"),
        "chat_id": ctx.peer_id - 2_000_000_000,
        "code": INVITE_CODE,
    };
    let response = match ctx.client.api_request::<i64>("execute", params).await {
        Ok(ok) => match ok {
            -100 => PxollyResponse::ErrorCode(-1),
            _ => PxollyResponse::Success,
        },
        Err(PxollyError::API(_)) => PxollyResponse::ErrorCode(0),
        _ => PxollyResponse::Text("internal".into()),
    };

    Ok(response)
}

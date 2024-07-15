use super::prelude::*;

pub struct InviteUser {
    pub(crate) vk_client: VKAPI,
}

#[async_trait::async_trait]
impl Handler for InviteUser {
    async fn handle(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = par! {
            "visible_messages_count": ctx.object.visible_messages_count.unwrap_or(0),
            "member_id": ctx.object.user_id.expect("Expect field: user_id"),
            "chat_id": ctx.peer_id().await? - 2_000_000_000,
            "code": EXECUTE_INVITE_CODE,
        };
        let response = match self.vk_client.api_request::<i64>("execute", params).await {
            Ok(ok) => match ok {
                -100 => PxollyResponse::ErrorCode(-1),
                _ => PxollyResponse::Success,
            },
            Err(WebhookError::VKAPI(_)) => PxollyResponse::ErrorCode(0),
            _ => PxollyResponse::Text("internal".into()),
        };

        Ok(response)
    }
}

const EXECUTE_INVITE_CODE: &str = r#"
if(API.friends.areFriends({user_ids:Args.member_id})[0].friend_status==3) {
    return API.messages.addChatUser({
        chat_id: Args.chat_id,
        user_id: Args.member_id,
        visible_messages_count: Args.visible_messages_count
    });
}
return -100;
"#;

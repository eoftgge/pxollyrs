use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::{PxollyErrorType, PxollyWebhookError};
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::execute::ExecuteParams;

pub struct InviteUser {
    pub(crate) vkontakte: VKontakteAPI,
}

impl Handler for InviteUser {
    const EVENT_TYPE: &'static str = "invite_user";

    async fn handle(
        &self,
        event: PxollyEvent,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let params = serde_json::json!({
            "visible_messages_count": event.object.visible_messages_count.unwrap_or(0),
            "member_id": event.object.user_id.expect("Expect field: user_id"),
            "chat_id": event.object.chat_local_id.unwrap(),
        });
        match self
            .vkontakte
            .execute::<i64>(ExecuteParams {
                code: EXECUTE_INVITE_CODE.into(),
                extras: params,
            })
            .await?
        {
            -100 => Err(PxollyWebhookError {
                message: None,
                error_type: PxollyErrorType::NotInFriends,
            }),
            _ => Ok(PxollyWebhookResponse::new(true)),
        }
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

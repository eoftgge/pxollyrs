use super::prelude::*;
use serde_json::Value;

pub struct SetTheme {
    pub(crate) api_client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for SetTheme {
    const EVENT_TYPE: &'static str = "set_theme";

    async fn execute(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id()?,
            "style": ctx.object.style.as_ref().expect("Expect field: style")
        };
        let response = match self
            .api_client
            .api_request::<Value>("messages.setConversationStyle", params)
            .await
        {
            Ok(_) => PxollyResponse::Success,
            Err(WebhookError::VKAPI(_)) => PxollyResponse::ErrorCode(0),
            _ => PxollyResponse::ErrorCode(2),
        };

        Ok(response)
    }
}

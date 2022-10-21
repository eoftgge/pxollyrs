use super::prelude::*;

pub struct ResetTheme {
    pub(crate) vk_client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for ResetTheme {
    const EVENT_TYPE: &'static str = "reset_theme";

    async fn execute(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id().await?
        };

        self.vk_client
            .api_request("messages.resetConversationStyle", params)
            .await?;
        Ok(PxollyResponse::Success)
    }
}

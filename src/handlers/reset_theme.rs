use super::prelude::*;

pub struct ResetTheme {
    pub(crate) vk_client: VKClient,
}

impl Handler for ResetTheme {
    const EVENT_TYPE: &'static str = "reset_theme";
    
    async fn handle(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = serde_json::json!({
            "peer_id": ctx.peer_id().await?,
        });

        self.vk_client
            .api_request("messages.resetConversationStyle", params)
            .await?;
        Ok(PxollyResponse::Success)
    }
}

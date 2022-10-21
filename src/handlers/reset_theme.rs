use super::prelude::*;

pub struct ResetTheme {
    pub(crate) api_client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for ResetTheme {
    const EVENT_TYPE: &'static str = "reset_theme";

    async fn execute(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id()?
        };

        self.api_client
            .api_request("messages.resetConversationStyle", params)
            .await?;
        Ok(PxollyResponse::Success)
    }
}

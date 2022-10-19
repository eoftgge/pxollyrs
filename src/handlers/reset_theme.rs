use super::prelude::*;

pub struct ResetTheme {
    pub(crate) client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for ResetTheme {
    const EVENT_TYPE: &'static str = "reset_theme";

    async fn execute(&self, ctx: PxollyContext) -> PxollyResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id()?
        };

        self.client
            .api_request("messages.resetConversationStyle", params)
            .await?;
        Ok(PxollyResponse::Success)
    }
}

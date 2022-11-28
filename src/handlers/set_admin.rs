use super::prelude::*;

pub struct SetAdmin {
    pub(crate) vk_client: VKAPI,
}

#[async_trait::async_trait]
impl Handler for SetAdmin {
    const EVENT_TYPE: &'static str = "set_admin";

    async fn handle(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id().await?,
            "role": if ctx.object.admin.expect("Expect field: admin") == 1 { "admin" } else { "member" },
            "user_id": ctx.object.user_id.expect("Expect field: user_id"),
        };
        self.vk_client
            .api_request("messages.setRole", params)
            .await?;

        Ok(PxollyResponse::Success)
    }
}

use super::prelude::*;

pub struct SetAdmin {
    pub(crate) client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for SetAdmin {
    const EVENT_TYPE: &'static str = "set_admin";

    async fn execute(&self, ctx: PxollyContext) -> PxollyResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id()?,
            "role": if ctx.object.admin.expect("Expect field: admin") == 1 { "admin" } else { "member" },
            "user_id": ctx.object.user_id.expect("Expect field: user_id"),
        };
        self.client.api_request("messages.setRole", params).await?;

        Ok(PxollyResponse::Success)
    }
}

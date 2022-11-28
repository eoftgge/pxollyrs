use super::prelude::*;

pub struct ChatMembers {
    pub(crate) vk_client: VKAPI,
}

#[async_trait::async_trait]
impl Handler for ChatMembers {
    const EVENT_TYPE: &'static str = "chat_members";

    async fn handle(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = par! {
            "chat_id": ctx.peer_id().await? - 2_000_000_000,
        };
        let response = self
            .vk_client
            .api_request::<serde_json::Value>("messages.getChat", params)
            .await?;
        let users = response
            .get("users")
            .expect("Expect field: users")
            .as_array()
            .expect("Expect field: users_array");

        Ok(PxollyResponse::Text(
            users
                .iter()
                .map(|x| x.as_str().unwrap_or(""))
                .collect::<Vec<_>>()
                .join(","),
        ))
    }
}

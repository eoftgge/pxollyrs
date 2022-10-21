use super::prelude::*;
use serde_json::Value;

pub struct GroupBan {
    pub(crate) api_client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for GroupBan {
    const EVENT_TYPE: &'static str = "group_ban";

    async fn execute(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = par! {
            "end_date": ctx.object.expired.expect("Expect field: end_date"),
            "group_id": ctx.object.group_id.expect("Expect field: group_id"),
            "owner_id": ctx.object.user_id.expect("Expect field: user_id")
        };

        let response = match self
            .api_client
            .api_request::<Value>("groups.ban", params)
            .await
        {
            Ok(_) => PxollyResponse::Success,
            Err(WebhookError::VKAPI(err)) => match err.error_code {
                15 => PxollyResponse::ErrorCode(0),
                _ => PxollyResponse::ErrorCode(-1),
            },
            _ => PxollyResponse::ErrorCode(2),
        };

        Ok(response)
    }
}

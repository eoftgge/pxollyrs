use super::prelude::*;
use serde_json::Value;

pub struct GroupUnban {
    pub(crate) client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for GroupUnban {
    const EVENT_TYPE: &'static str = "group_unban";

    async fn execute(&self, ctx: PxollyContext) -> PxollyResult<PxollyResponse> {
        let params = par! {
            "group_id": ctx.object.group_id.expect("Expect field: group_id"),
            "owner_id": ctx.object.user_id.expect("Expect field: owner_id")
        };
        let response = match self
            .client
            .api_request::<Value>("groups.unban", params)
            .await
        {
            Ok(_) => PxollyResponse::Success,
            Err(WebhookError::API(err)) => match err.error_code {
                15 => PxollyResponse::ErrorCode(0),
                _ => PxollyResponse::ErrorCode(2),
            },
            _ => PxollyResponse::ErrorCode(2),
        };

        Ok(response)
    }
}

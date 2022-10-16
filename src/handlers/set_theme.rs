use super::prelude::*;
use serde_json::Value;

pub struct SetTheme {
    pub(crate) client: APIClient,
}

#[async_trait::async_trait]
impl TraitHandler for SetTheme {
    const EVENT_TYPE: &'static str = "set_theme";

    async fn execute(&self, ctx: PxollyContext) -> PxollyResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id()?,
            "style": ctx.object.style.as_ref().expect("Expect field: style")
        };
        let response = match self
            .client
            .api_request::<Value>("messages.setConversationStyle", params)
            .await
        {
            Ok(_) => PxollyResponse::Success,
            Err(PxollyError::API(_)) => PxollyResponse::ErrorCode(0),
            _ => PxollyResponse::ErrorCode(2),
        };

        Ok(response)
    }
}

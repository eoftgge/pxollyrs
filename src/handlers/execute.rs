use super::prelude::*;
use serde_json::{to_string, Value};

pub struct Execute {
    pub(crate) api_client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for Execute {
    const EVENT_TYPE: &'static str = "execute";

    async fn execute(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = par! {
            "code": ctx.object.code.as_ref().expect("Expect field: code"),
            "chat_id": ctx.peer_id()?,
            "v": ctx.object.version.as_ref().expect("Expect field: version"),
        };

        let response = match self
            .api_client
            .api_request::<Value>("execute", params)
            .await
        {
            Ok(response) => PxollyResponse::Text(to_string(&response)?),
            Err(WebhookError::VKAPI(err)) => PxollyResponse::Text(to_string(&err)?),
            _ => PxollyResponse::ErrorCode(1),
        };

        Ok(response)
    }
}

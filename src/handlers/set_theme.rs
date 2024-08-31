use super::prelude::*;
use crate::vkontakte::responses::VKAPIError;
use serde_json::Value;

pub struct SetTheme {
    pub(crate) vk_client: VKClient,
}

impl Handler for SetTheme {
    const EVENT_TYPE: &'static str = "set_theme";

    async fn handle(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let params = serde_json::json!({
            "peer_id": ctx.peer_id().await?,
            "style": ctx.object.style.as_ref().expect("Expect field: style")
        });

        let response = match self
            .vk_client
            .api_request::<Value>("messages.setConversationStyle", params)
            .await
        {
            Ok(_) => PxollyResponse::Success,
            Err(WebhookError::VKAPI(VKAPIError { error_code, .. })) => match error_code {
                966 => PxollyResponse::ErrorCode(-1),
                _ => PxollyResponse::ErrorCode(0),
            },
            _ => PxollyResponse::ErrorCode(2),
        };

        Ok(response)
    }
}

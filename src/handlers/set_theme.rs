use crate::errors::PxollyError;
use crate::handlers::HandlerContext;
use crate::pxolly::types::responses::PxollyResponse;
use crate::{par, PxollyResult};
use serde_json::Value;

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let params = par! {
        "peer_id": ctx.peer_id,
        "style": ctx.object.style.as_ref().expect("Expect field: style")
    };
    let response = match ctx
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

use crate::handlers::HandlerContext;
use crate::pxolly::types::responses::PxollyResponse;
use crate::{par, PxollyResult};

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let params = par! {
        "peer_id": ctx.peer_id
    };

    ctx.client
        .api_request("messages.resetConversationStyle", params)
        .await?;
    Ok(PxollyResponse::Success)
}

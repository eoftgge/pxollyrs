use crate::handlers::HandlerContext;
use crate::pxolly::types::responses::PxollyResponse;
use crate::{par, PxollyResult};

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let params = par! {
        "peer_id": ctx.peer_id,
        "role": if ctx.object.admin.expect("Expect field: admin") == 1 { "admin" } else { "member" },
        "user_id": ctx.object.user_id.expect("Expect field: user_id"),
    };

    ctx.client.api_request("messages.setRole", params).await?;

    Ok(PxollyResponse::Success)
}

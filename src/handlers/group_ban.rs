use crate::errors::PxollyError;
use crate::handlers::HandlerContext;
use crate::pxolly::types::responses::PxollyResponse;
use crate::{par, PxollyResult};
use serde_json::Value;

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let params = par! {
        "end_date": ctx.object.expired.expect("Expect field: end_date"),
        "group_id": ctx.object.group_id.expect("Expect field: group_id"),
        "owner_id": ctx.object.user_id.expect("Expect field: user_id")
    };
    let response = match ctx.client.api_request::<Value>("groups.ban", params).await {
        Ok(_) => PxollyResponse::Success,
        Err(PxollyError::API(err)) => match err.error_code {
            15 => PxollyResponse::ErrorCode(0),
            _ => PxollyResponse::ErrorCode(-1),
        },
        _ => PxollyResponse::ErrorCode(2),
    };

    Ok(response)
}

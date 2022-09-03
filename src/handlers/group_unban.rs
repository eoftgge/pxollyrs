use crate::errors::PxollyError;
use crate::handlers::HandlerContext;
use crate::utils::models::PxollyResponse;
use crate::{par, PxollyResult};
use serde_json::Value;

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let params = par! {
        "group_id": ctx.object.group_id.expect("Expect field: group_id"),
        "owner_id": ctx.object.user_id.expect("Expect field: owner_id")
    };
    let response = match ctx
        .client
        .api_request::<Value>("groups.unban", params)
        .await
    {
        Ok(_) => PxollyResponse::Success,
        Err(PxollyError::API(err)) => match err.error_code {
            15 => PxollyResponse::ErrorCode(0),
            _ => PxollyResponse::ErrorCode(2),
        },
        _ => PxollyResponse::ErrorCode(2),
    };

    Ok(response)
}

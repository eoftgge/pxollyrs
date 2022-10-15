use crate::errors::PxollyError;
use crate::handlers::HandlerContext;
use crate::pxolly::types::responses::PxollyResponse;
use crate::{par, PxollyResult};
use serde_json::{to_string, Value};

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let params = par! {
        "code": ctx.object.code.as_ref().expect("Expect field: code"),
        "chat_id": ctx.peer_id,
        "v": ctx.object.version.as_ref().expect("Expect field: version"),
    };

    let result = match ctx.client.api_request::<Value>("execute", params).await {
        Ok(response) => PxollyResponse::Text(to_string(&response)?),
        Err(PxollyError::API(err)) => PxollyResponse::Text(to_string(&err)?),
        _ => PxollyResponse::ErrorCode(1),
    };

    Ok(result)
}

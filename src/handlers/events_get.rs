use crate::handlers::HANDLERS;
use crate::pxolly::types::responses::PxollyResponse;
use crate::PxollyResult;

pub async fn execute() -> PxollyResult<PxollyResponse> {
    Ok(PxollyResponse::Text(HANDLERS.join(",")))
}

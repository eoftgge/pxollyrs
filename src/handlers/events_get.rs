use crate::handlers::HANDLERS;
use crate::utils::models::PxollyResponse;
use crate::PxollyResult;

pub async fn execute() -> PxollyResult<PxollyResponse> {
    Ok(PxollyResponse::Text(HANDLERS.join(",")))
}

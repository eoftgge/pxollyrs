use super::types::responses::PxollyResponse;
use crate::pxolly::context::HandlerContext;
use crate::PxollyResult;

#[async_trait::async_trait]
pub trait TraitHandler: Send + Sync {
    const EVENT_TYPE: &'static str;

    async fn execute(&self, ctx: HandlerContext) -> PxollyResult<PxollyResponse>;
}

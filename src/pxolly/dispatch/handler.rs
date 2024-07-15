use crate::pxolly::dispatch::context::PxollyContext;
use crate::pxolly::types::responses::PxollyResponse;
use crate::WebhookResult;

#[async_trait::async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse>;
}

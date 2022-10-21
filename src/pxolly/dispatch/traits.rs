use crate::pxolly::dispatch::context::PxollyContext;
use crate::pxolly::types::responses::PxollyResponse;
use crate::WebhookResult;

#[async_trait::async_trait]
pub trait TraitHandler: Send + Sync {
    const EVENT_TYPE: &'static str;

    async fn execute(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse>;
}

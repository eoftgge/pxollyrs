use std::future::Future;
use crate::pxolly::dispatch::context::PxollyContext;
use crate::pxolly::types::responses::PxollyResponse;
use crate::WebhookResult;

pub trait Handler: Send + Sync {
    const EVENT_TYPE: &'static str;

    fn handle(&self, ctx: PxollyContext) -> impl Future<Output = WebhookResult<PxollyResponse>> + Send + Sync;
}

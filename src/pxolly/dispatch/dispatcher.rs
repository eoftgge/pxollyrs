use std::future::Future;
use crate::pxolly::dispatch::handler::Handler;
use std::sync::Arc;
use crate::handlers::prelude::{PxollyContext, PxollyResponse};
use crate::WebhookResult;

pub trait Dispatch {
    fn dispatch(
        &self,
        ctx: PxollyContext,
    ) -> impl Future<Output = WebhookResult<PxollyResponse>> + Send + Sync;
}

#[derive(Clone)]
pub struct DispatcherBuilder;

#[derive(Clone)]
pub struct Dispatcher<Current: Handler, Tail: Clone> {
    pub(crate) current: Arc<Current>,
    pub(crate) tail: Tail,
}

impl<Current, Tail> Dispatch for Dispatcher<Current, Tail>
where
    Current: Handler,
    Tail: Dispatch,
{
    async fn dispatch(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let event_type = Current::EVENT_TYPE;
        if ctx.event_type == event_type {
            return self.current.handle(ctx).await;
        }
        self.tail.dispatch(ctx).await
    }
}
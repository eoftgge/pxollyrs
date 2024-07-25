use std::future::Future;
use crate::pxolly::dispatch::handler::Handler;
use crate::handlers::prelude::{PxollyContext, PxollyResponse};
use crate::WebhookResult;

pub trait Dispatch: Send + Sync + 'static {
    fn dispatch(
        &self,
        ctx: PxollyContext,
    ) -> impl Future<Output = WebhookResult<PxollyResponse>> + Send + Sync;
}

#[derive(Clone)]
pub struct DispatcherBuilder;

pub struct Dispatcher<Current, Tail>
where
    Current: Handler,
    Tail: Dispatch,
{
    pub(crate) current: Current,
    pub(crate) tail: Tail,
}

impl Dispatch for DispatcherBuilder {
    async fn dispatch(&self, _: PxollyContext) -> WebhookResult<PxollyResponse> {
        Ok(PxollyResponse::Locked) // TODO: implement not found handler
    }
}

impl<Current, Tail> Dispatch for Dispatcher<Current, Tail>
where
    Current: Handler + Send + Sync,
    Tail: Dispatch + Send + Sync,
{
    async fn dispatch(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let event_type = Current::EVENT_TYPE;
        if ctx.event_type == event_type {
            return self.current.handle(ctx).await;
        }
        self.tail.dispatch(ctx).await
    }
}
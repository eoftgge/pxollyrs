use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::event::PxollyEvent;
use crate::pxolly::types::responses::errors::{PxollyErrorType, PxollyWebhookError};
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use std::future::Future;

pub trait Dispatch: Send + Sync + 'static {
    fn dispatch(
        &self,
        event: PxollyEvent,
    ) -> impl Future<Output = Result<PxollyWebhookResponse, PxollyWebhookError>> + Send + Sync;
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
    async fn dispatch(&self, _: PxollyEvent) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        Err(PxollyWebhookError {
            error_type: PxollyErrorType::UnknownEvent,
            message: None,
        })
    }
}

impl<Current, Tail> Dispatch for Dispatcher<Current, Tail>
where
    Current: Handler + Send + Sync,
    Tail: Dispatch + Send + Sync,
{
    async fn dispatch(
        &self,
        event: PxollyEvent,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let event_type = Current::EVENT_TYPE;
        if event.event_type == event_type {
            let object = match event.object.deserialize_into() {
                Ok(t) => t,
                Err(err) => {
                    log::error!("Failed to deserialize object: {:?}", err);
                    return Err(PxollyWebhookError {
                        message: Some(err.to_string()),
                        error_type: PxollyErrorType::InternalServerError,
                    });
                }
            };
            return self.current.handle(object).await;
        }
        self.tail.dispatch(event).await
    }
}

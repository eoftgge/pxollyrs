use std::future::Future;
use crate::pxolly::dispatch::handler::Handler;
use crate::database::conn::DatabaseConnection;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::{PxollyErrorType, PxollyWebhookError};
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;

pub trait Dispatch: Send + Sync + 'static {
    fn dispatch(
        &self,
        event: PxollyEvent,
        database: DatabaseConnection,
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
    async fn dispatch(&self, _: PxollyEvent, _: DatabaseConnection) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
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
    async fn dispatch(&self, event: PxollyEvent, database: DatabaseConnection) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let event_type = Current::EVENT_TYPE;
        if event.event_type == event_type {
            return self.current.handle(event, database).await;
        }
        self.tail.dispatch(event, database).await
    }
}

use crate::pxolly::types::events::event_type::EventType;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use serde::de::DeserializeOwned;
use std::future::Future;

pub trait Handler: Send + Sync + 'static {
    const EVENT_TYPE: EventType;
    type EventObject: DeserializeOwned;

    fn handle(
        &self,
        object: Self::EventObject,
    ) -> impl Future<Output = Result<PxollyWebhookResponse, PxollyWebhookError>> + Send + Sync;
}

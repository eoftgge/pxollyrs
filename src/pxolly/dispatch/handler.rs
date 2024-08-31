use std::future::Future;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;

pub trait Handler: Send + Sync + 'static {
    const EVENT_TYPE: &'static str;

    fn handle(
        &self,
        event: PxollyEvent,
    ) -> impl Future<Output = Result<PxollyWebhookResponse, PxollyWebhookError>> + Send + Sync;
}

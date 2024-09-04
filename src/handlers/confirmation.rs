use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::event_type::EventType;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfirmationObject {
    date: u64,
}

pub struct Confirmation {
    pub(crate) confirmation_code: String,
}

impl Handler for Confirmation {
    const EVENT_TYPE: EventType = EventType::Confirmation;
    type EventObject = ConfirmationObject;

    async fn handle(
        &self,
        _: Self::EventObject,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        Ok(PxollyWebhookResponse::new(true).code(self.confirmation_code.clone()))
    }
}

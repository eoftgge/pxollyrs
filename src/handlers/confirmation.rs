use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;

pub struct Confirmation {
    pub(crate) confirmation_code: String,
}

impl Handler for Confirmation {
    const EVENT_TYPE: &'static str = "confirmation";

    async fn handle(&self, _: PxollyEvent) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        Ok(PxollyWebhookResponse::new(true).code(self.confirmation_code.clone()))
    }
}

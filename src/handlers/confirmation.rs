use super::prelude::*;

pub struct Confirmation {
    pub(crate) confirmation_code: String,
}

impl Handler for Confirmation {
    const EVENT_TYPE: &'static str = "confirmation";
    
    async fn handle(&self, _: PxollyContext) -> WebhookResult<PxollyResponse> {
        Ok(PxollyResponse::ConfirmationCode(
            self.confirmation_code.clone(),
        ))
    }
}

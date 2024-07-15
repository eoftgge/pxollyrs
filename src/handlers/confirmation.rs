use super::prelude::*;

pub struct Confirmation {
    pub(crate) confirmation_code: String,
}

#[async_trait::async_trait]
impl Handler for Confirmation {
    async fn handle(&self, _: PxollyContext) -> WebhookResult<PxollyResponse> {
        Ok(PxollyResponse::ConfirmationCode(
            self.confirmation_code.clone(),
        ))
    }
}

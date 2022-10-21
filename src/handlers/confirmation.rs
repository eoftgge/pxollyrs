use super::prelude::*;

pub struct Confirmation {
    pub(crate) confirmation_code: String,
}

#[async_trait::async_trait]
impl TraitHandler for Confirmation {
    const EVENT_TYPE: &'static str = "confirmation";

    async fn execute(&self, _: PxollyContext) -> WebhookResult<PxollyResponse> {
        Ok(PxollyResponse::ConfirmationCode(
            self.confirmation_code.clone(),
        ))
    }
}

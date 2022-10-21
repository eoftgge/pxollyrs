use super::prelude::*;
use crate::pxolly::api::PxollyAPI;

pub struct Confirmation {
    pub(crate) pxolly_client: PxollyAPI,
}

#[async_trait::async_trait]
impl TraitHandler for Confirmation {
    const EVENT_TYPE: &'static str = "confirmation";

    async fn execute(&self, _: PxollyContext) -> WebhookResult<PxollyResponse> {
        let settings = self.pxolly_client.callback().get_settings().await?;
        Ok(PxollyResponse::ConfirmationCode(settings.confirmation_code))
    }
}

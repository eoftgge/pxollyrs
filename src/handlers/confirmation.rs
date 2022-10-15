use crate::pxolly::context::HandlerContext;
use crate::pxolly::traits::TraitHandler;
use crate::pxolly::types::responses::PxollyResponse;
use crate::PxollyResult;

pub struct Confirmation {
    pub(crate) confirmation_code: String,
}

#[async_trait::async_trait]
impl TraitHandler for Confirmation {
    const EVENT_TYPE: &'static str = "confirmation";

    async fn execute(&self, _: HandlerContext) -> PxollyResult<PxollyResponse> {
        Ok(PxollyResponse::ConfirmationCode(
            self.confirmation_code.clone(),
        ))
    }
}

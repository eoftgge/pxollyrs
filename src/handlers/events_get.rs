use super::prelude::*;

pub struct EventsGet {
    pub(crate) handlers: Vec<&'static str>,
}

#[async_trait::async_trait]
impl TraitHandler for EventsGet {
    const EVENT_TYPE: &'static str = "events_get";

    async fn execute(&self, _: PxollyContext) -> PxollyResult<PxollyResponse> {
        Ok(PxollyResponse::Text(self.handlers.join(",")))
    }
}

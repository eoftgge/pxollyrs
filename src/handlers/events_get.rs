use super::prelude::*;

pub struct EventsGet {
    pub(crate) handlers: Vec<&'static str>,
}

#[async_trait::async_trait]
impl Handler for EventsGet {
    const EVENT_TYPE: &'static str = "events_get";

    async fn handle(&self, _: PxollyContext) -> WebhookResult<PxollyResponse> {
        Ok(PxollyResponse::Text(self.handlers.join(",")))
    }
}

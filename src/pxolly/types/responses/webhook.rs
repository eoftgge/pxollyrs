use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PxollyWebhookResponse {
    ok: bool,
    conversation_message_ids: Option<Vec<u64>>,
    local_id: Option<u64>,
    code: Option<String>,
}

impl PxollyWebhookResponse {
    pub fn new(ok: bool) -> Self {
        Self {
            ok,
            conversation_message_ids: None,
            local_id: None,
            code: None,
        }
    }

    pub fn conversation_message_ids(mut self, conversation_message_ids: Vec<u64>) -> Self {
        self.conversation_message_ids = Some(conversation_message_ids);
        self
    }

    pub fn local_id(mut self, local_id: u64) -> Self {
        self.local_id = Some(local_id);
        self
    }

    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
}

impl IntoResponse for PxollyWebhookResponse {
    fn into_response(self) -> Response {
        let json = Json(self);
        Json::into_response(json)
    }
}

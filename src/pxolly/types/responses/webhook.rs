use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PxollyWebhookResponse {
    ok: bool,
    conversation_message_ids: Option<Vec<i64>>,
    local_id: Option<i64>,
}

impl PxollyWebhookResponse {
    pub fn new(ok: bool) -> Self {
        Self {
            ok, conversation_message_ids: None, local_id: None,
        }
    }
    
    pub fn conversation_message_ids(mut self, conversation_message_ids: Vec<i64>) -> Self {
        self.conversation_message_ids = Some(conversation_message_ids);
        self
    }
    
    pub fn local_id(mut self, local_id: i64) -> Self {
        self.local_id = Some(local_id);
        self
    }
}

impl IntoResponse for PxollyWebhookResponse {
    fn into_response(self) -> Response {
        let json = Json(self);
        json.into_response()
    }
}
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResetConversationStyleParams {
    peer_id: i64,
}
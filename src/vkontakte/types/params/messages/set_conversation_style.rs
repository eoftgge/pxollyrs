use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SetConversationStyleParams {
    peer_id: i64,
    style: String,
}
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SetConversationStyleParams {
    pub(crate) peer_id: i64,
    pub(crate) style: String,
}

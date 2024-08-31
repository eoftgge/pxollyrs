use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResetConversationStyleParams {
    pub(crate) peer_id: i64,
}

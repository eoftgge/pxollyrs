use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageDeleteResponse {
    pub(crate) peer_id: i64,
    pub(crate) conversation_message_id: u64,
    pub(crate) response: u8,
}

pub type MessagesDeleteResponse = Vec<MessageDeleteResponse>;

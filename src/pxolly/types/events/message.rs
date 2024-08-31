use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyMessage {
    pub date: u64,
    pub conversation_message_id: u64,
    pub from_id: i64,
    pub text: String,
}

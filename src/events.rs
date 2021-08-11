use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyData {
    pub from_id: Option<i64>,
    pub chat_id: Option<String>,
    pub chat_uid: Option<u64>,
    pub conversation_message_ids: Option<Vec<u64>>,
    pub is_spam: Option<u8>,
    pub user_id: Option<i64>,
    pub visible_messages_count: Option<u16>,
    pub payload: Option<String>,
    pub expired: Option<i64>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyEvent {
    #[serde(rename = "type")]
    pub ctype: String,
    pub secret_key: String,
    pub object: PxollyData,
}

pub enum PxollyResponse {
    Success = 1,
    Fail = 0,
    UnknownErrorOrError = -1,
    UnknownUIDOrNoSupport = -2,
}

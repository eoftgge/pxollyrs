use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyUser {
    pub id: i64,
    pub role: u16, // ???
    pub balance: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyMessage {
    pub date: u64,
    pub conversation_message_id: u64,
    pub from_id: i64,
    pub text: String,
}

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
    pub style: Option<String>,
    pub user: Option<PxollyUser>,
    pub message: Option<PxollyMessage>,
    pub success: Option<String>,
    pub can_text: Option<u8>,
    pub admin: Option<u8>,
    pub code: Option<String>,

    #[serde(rename = "v")]
    pub version: Option<u8>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyEvent {
    #[serde(rename = "type")]
    pub c_type: String,
    pub secret_key: String,
    pub event_id: String,
    pub from_id: Option<i64>,
    pub user_id: Option<i64>,
    pub object: PxollyData,
}

/// Responses:
/// Text(String) -> return text (for example: Text("API.messages.send({ ... });"))
/// ConfirmationCode(String) -> return code for confirm
/// ErrorCode(u8) -> return error code
/// Success -> return 1
/// Locked -> ip isn't pxolly
#[derive(Debug)]
pub enum PxollyResponse {
    Text(String),
    ConfirmationCode(String),
    ErrorCode(i8),
    Success,
    Locked,
}

impl ToString for PxollyResponse {
    fn to_string(&self) -> String {
        match self {
            Self::Text(text) => text.into(),
            Self::ConfirmationCode(code) => code.into(),
            Self::ErrorCode(code) => code.to_string(),
            Self::Success => "1".into(),
            Self::Locked => "locked".into(),
        }
    }
}

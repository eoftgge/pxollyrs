use serde::Deserialize;
use crate::pxolly::types::events::message::PxollyMessage;
use crate::pxolly::types::events::user::PxollyUser;

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyObject {
    pub date: Option<i64>,
    pub chat_local_id: Option<u32>,
    pub from_id: Option<i64>,
    pub prefix: Option<String>,
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
    pub is_remove: Option<u8>,
    pub photo_url: Option<String>,

    #[serde(rename = "v")]
    pub version: Option<u8>,
}
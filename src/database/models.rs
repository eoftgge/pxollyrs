use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PxollyChatModel {
    chat_id: String,
    chat_uid: String,
}

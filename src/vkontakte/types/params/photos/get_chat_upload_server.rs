use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetChatUploadServerParams {
    pub(crate) chat_id: u64,
}
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetChatUploadServerParams {
    chat_id: u64,
}
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetChatUploadServerResponse {
    upload_url: String,
}
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetChatUploadServerResponse {
    pub upload_url: String,
}

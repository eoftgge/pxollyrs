use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SetChatPhotoParams {
    file: String,
}
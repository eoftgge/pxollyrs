use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SetChatPhotoParams {
    pub(crate) file: String,
}
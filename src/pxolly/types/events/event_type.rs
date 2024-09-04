use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum EventType {
    ChatPhotoUpdate,
    Confirmation,
    DeleteForAll,
    InviteUser,
    ResetTheme,
    SetTheme,
    Sync,
}
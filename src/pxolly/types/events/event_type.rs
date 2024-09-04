use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
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
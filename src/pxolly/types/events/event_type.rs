use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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
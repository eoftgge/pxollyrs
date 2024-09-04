use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    ChatPhotoUpdate,
    Confirmation,
    DeleteForAll,
    InviteUser,
    ResetTheme,
    SetTheme,
    Sync,
}

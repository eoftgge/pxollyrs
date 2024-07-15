use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EditSettingsResponse {
    pub state: u16,
}
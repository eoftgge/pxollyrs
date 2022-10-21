use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetSettingsResponse {
    pub is_available: u8,
    pub method: u8,
    pub url: String,
    pub confirmation_code: String,
    pub version: u8,
    pub secret_key: String,
}

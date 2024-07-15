use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetSettingsResponse {
    pub enabled: u8,
    pub types: Option<Vec<String>>,
    pub url: String,
    pub confirm_code: String,
    pub secret_key: String,
}

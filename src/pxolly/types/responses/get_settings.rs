use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetSettingsResponse {
    pub enabled: bool,
    pub types: Vec<String>,
    pub url: String,
    pub confirm_code: String,
    pub secret_key: String,
}

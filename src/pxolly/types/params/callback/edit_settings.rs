use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EditSettingsParams {
    pub url: Option<String>,
    pub secret_key: Option<String>,
    pub is_hidden: bool,
}

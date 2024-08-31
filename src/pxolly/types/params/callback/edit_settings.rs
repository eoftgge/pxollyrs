use serde::Serialize;

#[derive(Serialize)]
pub struct EditSettingsParams {
    pub url: Option<String>,
    pub secret_key: Option<String>,
    pub is_hidden: bool,
}

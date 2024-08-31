use serde::Serialize;

#[derive(Serialize)]
pub struct EditSettingsParams {
    url: Option<String>,
    secret_key: Option<String>,
    is_hidden: bool,
}
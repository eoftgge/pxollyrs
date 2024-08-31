use serde::Serialize;

#[derive(Serialize)]
pub struct GetSettingsParams {
    pub v: &'static str,
}
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetSettingsParams {
    pub v: &'static str,
}

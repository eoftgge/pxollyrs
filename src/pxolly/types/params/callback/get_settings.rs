use serde::Serialize;

#[derive(Serialize)]
pub struct GetSettingsParams {
    v: &'static str,
}
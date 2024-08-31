use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ExecuteParams {
    pub(crate) code: String,
    #[serde(flatten)]
    pub(crate) extras: Value,
}
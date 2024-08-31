use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ExecuteParams {
    code: String,
    #[serde(flatten)]
    extras: Value,
}
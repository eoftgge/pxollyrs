use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PxollyAPIResponse<T> {
    Response(T),
    Error(PxollyAPIError),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PxollyAPIError {
    pub error_code: i32,
    pub error_msg: String,
    pub error_text: String,
}

#[derive(Serialize, Debug)]
pub struct PxollyAPIRequestParams<'a> {
    pub access_token: &'a str,
    pub format: &'a str,
    #[serde(flatten)]
    pub others: Value,
}

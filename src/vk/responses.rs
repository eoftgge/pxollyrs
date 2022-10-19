use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct VKAPIResponseError {
    pub error_code: i32,
    pub error_msg: String,
    pub request_params: Vec<Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VKAPIResponse<T> {
    Response(T),
    Error(VKAPIResponseError),
}

#[derive(Serialize, Debug)]
pub struct VKAPIRequestParams<'a> {
    pub access_token: &'a str,
    #[serde(rename = "v")]
    pub version: &'a str,
    #[serde(flatten)]
    pub others: Value,
}

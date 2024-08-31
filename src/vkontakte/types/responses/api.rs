use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VKontakteAPIResponse<T> {
    Response(T),
    Error(VKontakteAPIError),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VKontakteAPIError {
    pub error_code: i32,
    pub error_msg: String,
    pub request_params: Vec<Value>,
}

#[derive(Serialize, Debug)]
pub struct VKontakteAPIRequestParams<'a> {
    pub access_token: &'a str,
    #[serde(rename = "v")]
    pub version: &'a str,
    #[serde(flatten)]
    pub extras: Value,
}

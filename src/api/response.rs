use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum APIObjectResponse {
	Object(Map<String, Value>),
	Integer(i64),
}

#[derive(Deserialize, Debug)]
pub struct APIError {
	pub error_code: i32,
	pub error_msg: String,
	pub request_params: Vec<Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum APIResponse {
	Response(APIObjectResponse),
	Error(APIError)
}

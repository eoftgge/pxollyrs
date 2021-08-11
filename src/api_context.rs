use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Clone)]
pub struct APIClient {
    pub client: reqwest::Client,
    pub access_token: String,
    pub api_version: f32,
}

#[derive(Deserialize, Debug)]
pub struct APIErrorCMP {
    pub error_code: i32,
    pub error_msg: String,
    pub request_params: Vec<Value>,
}

#[derive(Deserialize, Debug)]
pub struct APIError {
    pub error: APIErrorCMP,
}

#[derive(Deserialize, Debug, Default)]
pub struct APIResponse {
    pub response: Option<Map<String, Value>>,
}

impl APIClient {
    pub fn new(access_token: String, api_version: f32) -> Self {
        Self {
            access_token,
            api_version,
            client: reqwest::Client::new(),
        }
    }

    pub async fn api_request(
        &self,
        method: &str,
        params: &mut HashMap<&str, String>,
    ) -> Result<APIResponse, APIError> {
        params.insert("access_token", self.access_token.to_string());
        params.insert("v", self.api_version.to_string());

        let mut response = self
            .client
            .post(format!("https://api.vk.com/method/{}", method))
            .form(&params)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if response
            .as_object_mut()
            .unwrap()
            .remove("response")
            .is_some()
        {
            let result: APIResponse =
                serde_json::from_value(response).unwrap_or(APIResponse::default());
            Ok(result)
        } else {
            let err: APIError = serde_json::from_value(response).unwrap();
            Err(err)
        }
    }
}

impl From<reqwest::Error> for APIError {
    fn from(_: reqwest::Error) -> Self {
        Self {
            error: APIErrorCMP {
                error_msg: "Error is `reqwest::Error`...".to_string(),
                error_code: 404,
                request_params: vec![],
            },
        }
    }
}

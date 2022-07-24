use crate::api::response::APIResponse;
use crate::errors::{PxollyError, PxollyResult};
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct APIClient {
    access_token: String,
    version: String,  // теряется точность если хранить в f32
    client: Client,
}

impl APIClient {
    pub fn new(access_token: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            access_token: access_token.into(),
            client: Client::new(),
        }
    }

    pub fn make_url(&self, method: String) -> String {
        format!("https://api.vk.com/method/{}", method)
    }

    pub fn make_params(&self, params: impl Serialize) -> PxollyResult<serde_json::Value> {
        let mut params = serde_json::to_value(params)?;
        let ref_params = params.as_object_mut().ok_or_else(|| PxollyError::from("Params isn't object"))?;
        ref_params.insert(
            "access_token".into(),
            serde_json::Value::from(&*self.access_token),
        );
        ref_params.insert("v".into(), serde_json::Value::from(&*self.version));

        Ok(params)
    }

    pub async fn api_request<Method, Params, Response>(
        &self,
        method: Method,
        params: Params,
    ) -> PxollyResult<Response> where
        Method: Into<String>,
        Params: Serialize,
        Response: DeserializeOwned
    {
        let request_builder = self
            .client
            .post(self.make_url(method.into()))
            .form(&self.make_params(params)?);

        let response = request_builder
            .send()
            .await?
            .json::<APIResponse<Response>>()
            .await?;

        match response {
            APIResponse::Response(response) => Ok(response),
            APIResponse::Error(error) => Err(PxollyError::API(error)),
        }
    }
}

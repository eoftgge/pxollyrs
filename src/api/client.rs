use crate::api::responses::{APIRequestParams, APIResponse};
use crate::errors::{PxollyError, PxollyResult};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct APIClient {
    access_token: Arc<str>,
    version: Arc<str>,
    client: Arc<Client>,
}

impl APIClient {
    pub fn new(access_token: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            access_token: Arc::from(access_token.into()),

            version: Arc::from(version.into()),
            client: Arc::new(Client::new()),
        }
    }

    pub fn create_url(&self, method_name: String) -> String {
        format!("https://api.vk.com/method/{}", method_name)
    }

    pub fn create_params(&self, params: impl Serialize) -> PxollyResult<APIRequestParams> {
        Ok(APIRequestParams {
            access_token: &*self.access_token,
            version: &*self.version,
            others: serde_json::to_value(params)?,
        })
    }

    pub async fn api_request<T>(
        &self,
        method: impl Into<String>,
        params: impl Serialize,
    ) -> PxollyResult<T>
    where
        T: DeserializeOwned + Debug,
    {
        let request_builder = self
            .client
            .post(self.create_url(method.into()))
            .form(&self.create_params(params)?);

        let response = request_builder
            .send()
            .await?
            .json::<APIResponse<T>>()
            .await?;

        log::debug!("Sent the request to VKAPI, response: {:?}", response);

        match response {
            APIResponse::Response(response) => Ok(response),
            APIResponse::Error(error) => Err(PxollyError::API(error)),
        }
    }
}

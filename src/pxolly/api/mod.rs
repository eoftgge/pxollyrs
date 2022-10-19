use std::fmt::Debug;
use std::sync::Arc;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::vk::responses::VKAPIRequestParams;
use crate::PxollyResult;

const API_URL: &str = "https://api.pxolly.ru/method/";

#[derive(Clone)]
pub struct PxollyAPI {
    client: Arc<Client>,
    access_token: Arc<str>
}

impl PxollyAPI {
    pub fn new(client: Arc<Client>, access_token: impl Into<String>) -> Self {
        Self {
            client,
            access_token: Arc::from(access_token.into())
        }
    }

    pub fn api_request<T: DeserializeOwned + Debug>(
        &self,
        method: impl Into<String>,
        params: impl Serialize
    ) -> PxollyResult<T> {
        let response = self
            .client
            .post(self.create_url(method.into()))
            .form(&self.create_params(params))
            .send()
            .await
            .json::<T>()
            .await?
    }

    fn create_url(&self, method_name: String) -> String {
        format!("{}{}", API_URL, method_name)
    }

    fn create_params(&self, params: impl Serialize) -> PxollyResult<VKAPIRequestParams> {
        Ok(VKAPIRequestParams {
            access_token: &*self.access_token,
            others: serde_json::to_value(params)?,
        })
    }
}
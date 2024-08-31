use crate::errors::{WebhookError, WebhookResult};
use crate::vkontakte::responses::{VKAPIRequestParams, VKAPIResponse};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct VKClient {
    access_token: Arc<str>,
    version: Arc<str>,
    client: Arc<Client>,
}

impl VKClient {
    pub fn new(
        client: Arc<Client>,
        access_token: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            client,
            access_token: Arc::from(access_token.into()),
            version: Arc::from(version.into()),
        }
    }

    pub async fn api_request<T: DeserializeOwned + Debug>(
        &self,
        method: impl Into<String>,
        params: impl Serialize,
    ) -> WebhookResult<T> {
        let response = self
            .client
            .post(self.create_url(method.into()))
            .form(&self.create_params(params)?)
            .send()
            .await?
            .json::<VKAPIResponse<T>>()
            .await?;

        log::debug!("sent the request to VK API, response: {:?}", response);

        match response {
            VKAPIResponse::Response(response) => Ok(response),
            VKAPIResponse::Error(error) => Err(WebhookError::VKAPI(error)),
        }
    }

    fn create_url(&self, method_name: String) -> String {
        format!("{}{}", API_URL, method_name)
    }

    fn create_params(&self, params: impl Serialize) -> WebhookResult<VKAPIRequestParams> {
        Ok(VKAPIRequestParams {
            access_token: &self.access_token,
            version: &self.version,
            others: serde_json::to_value(params)?,
        })
    }
}

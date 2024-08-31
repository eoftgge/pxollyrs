use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use crate::vkontakte::DEFAULT_API_URL_VKONTAKTE;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::responses::{VKontakteAPIRequestParams, VKontakteAPIResponse};

#[derive(Clone)]
pub struct VKontakteAPI {
    access_token: Arc<str>,
    version: Arc<str>,
    client: Client,
}

impl VKontakteAPI {
    pub fn new(
        client: Client,
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
    ) -> Result<T, VKontakteError> {
        let url = format!("{}{}", DEFAULT_API_URL_VKONTAKTE, method.into());
        let params = VKontakteAPIRequestParams {
            access_token: &self.access_token,
            version: &self.version,
            extras: serde_json::to_value(params)?
        };
        let response = self
            .client
            .post(url)
            .form(&self.create_params(params)?)
            .send()
            .await?
            .json::<VKontakteAPIResponse<T>>()
            .await?;

        log::debug!("sent the request to VK API, response: {:?}", response);

        match response {
            VKontakteAPIResponse::Response(response) => Ok(response),
            VKontakteAPIResponse::Error(error) => Err(VKontakteError::API(error)),
        }
    }
}

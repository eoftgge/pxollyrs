pub mod categories;
pub mod methods;
pub mod responses;

use crate::errors::WebhookError;
use crate::pxolly::api::responses::{PxollyAPIRequestParams, PxollyAPIResponse};
use crate::WebhookResult;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

const API_URL: &str = "https://api.pxolly.ru/m/";

#[derive(Clone)]
pub struct PxollyAPI {
    client: Arc<Client>,
    access_token: Arc<str>,
}

impl PxollyAPI {
    pub fn new(client: Arc<Client>, access_token: impl Into<String>) -> Self {
        Self {
            client,
            access_token: Arc::from(access_token.into()),
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
            .json::<PxollyAPIResponse<T>>()
            .await?;

        match response {
            PxollyAPIResponse::Response(ok) => Ok(ok),
            PxollyAPIResponse::Error(err) => Err(WebhookError::PxollyAPI(err)),
        }
    }

    fn create_url(&self, method_name: String) -> String {
        format!("{}{}", API_URL, method_name)
    }

    fn create_params(&self, params: impl Serialize) -> WebhookResult<PxollyAPIRequestParams> {
        Ok(PxollyAPIRequestParams {
            access_token: &*self.access_token,
            format: "json", // TODO: i maybe add msgpack format in the future
            others: serde_json::to_value(params)?,
        })
    }
}

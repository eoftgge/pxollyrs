pub mod categories;
pub mod methods;
pub mod responses;

use crate::errors::WebhookError;
use crate::pxolly::api::responses::{PxollyAPIRequestParams, PxollyAPIResponse};
use crate::WebhookResult;
use reqwest::header::HeaderValue;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

const API_URL: &str = "https://api.pxolly.com/m/";

pub fn create_url(method_name: String) -> String {
    format!("{}{}", API_URL, method_name)
}

pub async fn into_response<T: DeserializeOwned + Debug>(
    response: Response,
) -> WebhookResult<PxollyAPIResponse<T>> {
    let content_type = response.headers().get("Content-Type");

    if content_type.eq(&Some(&HeaderValue::from_static("application/x-msgpack"))) {
        let bytes = response.bytes().await?;
        Ok(rmp_serde::from_slice(&bytes)?)
    } else {
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

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
            .post(create_url(method.into()))
            .form(&self.create_params(params)?)
            .send()
            .await?;

        match into_response(response).await? {
            PxollyAPIResponse::Response(ok) => Ok(ok),
            PxollyAPIResponse::Error(err) => Err(WebhookError::PxollyAPI(err)),
        }
    }

    fn create_params(&self, params: impl Serialize) -> WebhookResult<PxollyAPIRequestParams> {
        Ok(PxollyAPIRequestParams {
            access_token: &self.access_token,
            format: "msgpack",
            others: serde_json::to_value(params)?,
        })
    }
}

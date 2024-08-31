use crate::pxolly::errors::PxollyError;
use crate::pxolly::types::requests::PxollyAPIRequestParams;
use crate::pxolly::types::responses::api::PxollyAPIResponse;
use crate::pxolly::DEFAULT_API_URL_PXOLLY;
use reqwest::header::HeaderValue;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

async fn into_response<T: DeserializeOwned + Debug>(
    response: Response,
) -> Result<PxollyAPIResponse<T>, PxollyError> {
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
    client: Client,
    access_token: Arc<str>,
}

impl PxollyAPI {
    pub fn new(client: Client, access_token: impl Into<String>) -> Self {
        Self {
            client,
            access_token: Arc::from(access_token.into()),
        }
    }

    pub async fn api_request<T: DeserializeOwned + Debug>(
        &self,
        method: impl Into<String>,
        params: impl Serialize + Debug,
    ) -> Result<T, PxollyError> {
        let url = format!("{}{}", DEFAULT_API_URL_PXOLLY, method.into());
        let params = PxollyAPIRequestParams {
            access_token: &self.access_token,
            format: "msgpack",
            extras: params,
        };
        let response = self.client.post(&url).form(&params).send().await?;
        log::debug!(
            "Got a response from @pxolly, content({}): {:?}",
            url,
            response
        );

        let response = into_response(response).await?;
        match response {
            PxollyAPIResponse::Response(ok) => Ok(ok),
            PxollyAPIResponse::Error(err) => Err(PxollyError::API(err)),
        }
    }
}

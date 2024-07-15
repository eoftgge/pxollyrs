use crate::pxolly::api::PxollyAPI;
use crate::pxolly::types::responses::edit_settings::EditSettingsResponse;
use crate::WebhookResult;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;

#[derive(Serialize)]
pub struct EditSettingsBuilder {
    url: Option<String>,
    secret_key: Option<String>,

    #[serde(skip)]
    api_client: PxollyAPI,
}

impl EditSettingsBuilder {
    pub(crate) fn new(api_client: PxollyAPI) -> Self {
        Self {
            api_client,
            url: None,
            secret_key: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn secret_key(mut self, secret_key: impl Into<String>) -> Self {
        self.secret_key = Some(secret_key.into());
        self
    }
}

impl std::future::IntoFuture for EditSettingsBuilder {
    type Output = WebhookResult<EditSettingsResponse>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let api_client = self.api_client.clone();
            api_client.api_request("callback.editSettings", self).await
        })
    }
}

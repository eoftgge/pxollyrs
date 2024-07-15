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
    is_hidden: bool,

    #[serde(skip)]
    api_client: PxollyAPI,
}

impl EditSettingsBuilder {
    pub(crate) fn new(api_client: PxollyAPI) -> Self {
        Self {
            api_client,
            url: None,
            secret_key: None,
            is_hidden: false,
        }
    }

    pub fn set_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn set_secret_key(mut self, secret_key: impl Into<String>) -> Self {
        self.secret_key = Some(secret_key.into());
        self
    }

    pub fn set_is_hidden(mut self, is_hidden: bool) -> Self {
        self.is_hidden = is_hidden;
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

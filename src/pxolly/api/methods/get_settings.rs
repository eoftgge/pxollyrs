use crate::pxolly::api::PxollyAPI;
use crate::pxolly::types::responses::get_settings::GetSettingsResponse;
use crate::WebhookResult;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;

#[derive(Serialize)]
pub struct GetSettingsBuilder {
    #[serde(skip)]
    api_client: PxollyAPI,
}

impl GetSettingsBuilder {
    pub(crate) fn new(api_client: PxollyAPI) -> Self {
        Self { api_client }
    }
}

impl std::future::IntoFuture for GetSettingsBuilder {
    type Output = WebhookResult<GetSettingsResponse>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let api_client = self.api_client.clone();
            api_client.api_request("callback.getSettings", self).await
        })
    }
}

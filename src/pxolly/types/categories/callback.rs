use crate::pxolly::types::params::{EditSettingsParams, GetSettingsParams};
use crate::pxolly::api::PxollyAPI;
use crate::pxolly::types::responses::{EditSettingsResponse, GetSettingsResponse};
use crate::WebhookResult;

pub struct CallbackMethods {
    api_client: PxollyAPI,
}

impl CallbackMethods {
    pub(crate) fn new(api_client: PxollyAPI) -> Self {
        Self { api_client }
    }

    pub async fn edit_settings(&self, params: EditSettingsParams) -> WebhookResult<EditSettingsResponse> {
        self.api_client.api_request("callback.editSettings", params).await
    }

    pub async fn get_settings(&self, params: GetSettingsParams) -> WebhookResult<GetSettingsResponse> {
        self.api_client.api_request("callback.getSettings", params).await
    }
}

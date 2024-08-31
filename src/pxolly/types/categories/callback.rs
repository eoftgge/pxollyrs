use crate::pxolly::types::params::{EditSettingsParams, GetSettingsParams};
use crate::pxolly::api::PxollyAPI;
use crate::pxolly::errors::PxollyError;
use crate::pxolly::types::responses::callback::{EditSettingsResponse, GetSettingsResponse};

pub struct CallbackMethods {
    api_client: PxollyAPI,
}

impl CallbackMethods {
    pub(crate) fn new(api_client: PxollyAPI) -> Self {
        Self { api_client }
    }

    pub async fn edit_settings(&self, params: EditSettingsParams) -> Result<EditSettingsResponse, PxollyError> {
        self.api_client.api_request("callback.editSettings", params).await
    }

    pub async fn get_settings(&self, params: GetSettingsParams) -> Result<GetSettingsResponse, PxollyError> {
        self.api_client.api_request("callback.getSettings", params).await
    }
}

use crate::pxolly::api::PxollyAPI;
use crate::pxolly::errors::PxollyError;
use crate::pxolly::types::params::{
    EditSettingsParams, GetSettingsParams, ImportChatLocalIdsParams,
};
use crate::pxolly::types::responses::callback::{
    EditSettingsResponse, GetSettingsResponse, ImportChatLocalIdsResponse,
};

pub struct CallbackMethods {
    api_client: PxollyAPI,
}

impl CallbackMethods {
    pub(crate) fn new(api_client: PxollyAPI) -> Self {
        Self { api_client }
    }

    pub async fn edit_settings(
        &self,
        params: EditSettingsParams,
    ) -> Result<EditSettingsResponse, PxollyError> {
        self.api_client
            .api_request("callback.editSettings", params)
            .await
    }

    pub async fn get_settings(
        &self,
        params: GetSettingsParams,
    ) -> Result<GetSettingsResponse, PxollyError> {
        self.api_client
            .api_request("callback.getSettings", params)
            .await
    }

    pub async fn import_chat_local_ids(
        &self,
        params: ImportChatLocalIdsParams,
    ) -> Result<ImportChatLocalIdsResponse, PxollyError> {
        self.api_client
            .api_request("callback.importChatLocalIds", params)
            .await
    }
}

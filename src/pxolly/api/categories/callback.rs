use crate::pxolly::api::methods::edit_settings::EditSettingsBuilder;
use crate::pxolly::api::methods::get_settings::GetSettingsBuilder;
use crate::pxolly::api::PxollyAPI;

pub struct CallbackMethods {
    api_client: PxollyAPI,
}

impl CallbackMethods {
    pub(crate) fn new(api_client: PxollyAPI) -> Self {
        Self { api_client }
    }

    pub fn edit_settings(self) -> EditSettingsBuilder {
        EditSettingsBuilder::new(self.api_client)
    }

    pub fn get_settings(self) -> GetSettingsBuilder {
        GetSettingsBuilder::new(self.api_client)
    }
}

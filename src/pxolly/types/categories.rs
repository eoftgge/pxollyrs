use crate::pxolly::api::PxollyAPI;
use crate::pxolly::types::categories::callback::CallbackMethods;

pub mod callback;

pub struct Categories {
    api_client: PxollyAPI
}

impl Categories {
    pub fn new(api_client: PxollyAPI) -> Self {
        Self { api_client }
    }
    
    pub fn callback(&self) -> CallbackMethods {
        CallbackMethods::new(self.api_client.clone())
    }
}
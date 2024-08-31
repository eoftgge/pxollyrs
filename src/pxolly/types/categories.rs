use crate::pxolly::api::PxollyAPI;
use crate::pxolly::types::categories::callback::CallbackMethods;

pub mod callback;

pub trait Categories {
    fn api_client(&self) -> PxollyAPI;

    fn callback(&self) -> CallbackMethods {
        CallbackMethods::new(self.api_client())
    }
}

impl Categories for PxollyAPI {
    fn api_client(&self) -> PxollyAPI {
        self.clone()
    }
}

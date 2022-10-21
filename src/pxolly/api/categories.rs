use crate::pxolly::api::PxollyAPI;

pub mod callback;

impl PxollyAPI {
    pub fn callback(&self) -> callback::CallbackMethods {
        callback::CallbackMethods::new(self.clone())
    }
}

use std::fmt::Debug;
use serde::de::DeserializeOwned;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::categories::messages::MessagesMethods;
use crate::vkontakte::types::categories::photos::PhotosMethods;
use crate::vkontakte::types::params::execute::ExecuteParams;

pub mod messages;
pub mod photos;


pub trait Categories {
    fn api_client(&self) -> VKontakteAPI;
    
    fn messages(&self) -> MessagesMethods {
        MessagesMethods::new(self.api_client())
    }
    
    fn photos(&self) -> PhotosMethods {
        PhotosMethods::new(self.api_client())
    }
    
    async fn execute<T: DeserializeOwned + Debug>(&self, params: ExecuteParams) -> Result<T, VKontakteError> {
        self.api_client().api_request("execute", params).await
    }
}

impl Categories for VKontakteAPI {
    fn api_client(&self) -> VKontakteAPI {
        self.clone()
    }
}
use std::fmt::Debug;
use serde::de::DeserializeOwned;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::categories::messages::MessagesMethods;
use crate::vkontakte::types::categories::photos::PhotosMethods;
use crate::vkontakte::types::params::execute::ExecuteParams;

pub mod messages;
pub mod photos;

pub struct Categories {
    api_client: VKontakteAPI,
}

impl Categories {
    pub fn new(api_client: VKontakteAPI) -> Self {
        Self { api_client }
    }
    
    pub fn messages(&self) -> MessagesMethods {
        MessagesMethods::new(self.api_client.clone())
    }
    
    pub fn photos(&self) -> PhotosMethods {
        PhotosMethods::new(self.api_client.clone())
    }
    
    pub async fn execute<T: DeserializeOwned + Debug>(&self, params: ExecuteParams) -> Result<T, VKontakteError> {
        self.api_client.api_request("execute", params).await
    }
}
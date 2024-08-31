use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::errors::VKontakteError;

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
    
    pub async fn execute(&self, params: ExecuteParams) -> Result<ExecuteResponse, VKontakteError> {
        self.api_client.api_request("execute", params).await
    }
}
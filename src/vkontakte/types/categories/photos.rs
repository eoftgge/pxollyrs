use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::params::photos::get_chat_upload_server::GetChatUploadServerParams;
use crate::vkontakte::types::responses::photos::get_chat_upload_server::GetChatUploadServerResponse;

pub struct PhotosMethods {
    api_client: VKontakteAPI
}

impl PhotosMethods {
    pub fn new(api_client: VKontakteAPI) -> Self {
        Self {
            api_client
        }
    }
    
    pub async fn get_chat_upload_server(&self, params: GetChatUploadServerParams) -> Result<GetChatUploadServerResponse, VKontakteError> {
        self.api_client.api_request("photos.getChatUploadServer", params).await
    }
}
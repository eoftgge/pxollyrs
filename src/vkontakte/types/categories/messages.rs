use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::errors::VKontakteError;
use crate::vkontakte::types::params::messages::delete::MessagesDeleteParams;
use crate::vkontakte::types::params::messages::reset_conversation_style::ResetConversationStyleParams;
use crate::vkontakte::types::params::messages::set_chat_photo::SetChatPhotoParams;
use crate::vkontakte::types::params::messages::set_conversation_style::SetConversationStyleParams;
use crate::vkontakte::types::responses::messages::delete::MessagesDeleteResponse;
use crate::vkontakte::types::responses::messages::reset_conversation_style::ResetConversationStyleResponse;
use crate::vkontakte::types::responses::messages::set_chat_photo::SetChatPhotoResponse;
use crate::vkontakte::types::responses::messages::set_conversation_style::SetConversationStyleResponse;

pub struct MessagesMethods {
    api_client: VKontakteAPI,
}

impl MessagesMethods {
    pub fn new(api_client: VKontakteAPI) -> Self {
        Self { api_client }
    }

    pub async fn delete(
        &self,
        params: MessagesDeleteParams,
    ) -> Result<MessagesDeleteResponse, VKontakteError> {
        self.api_client.api_request("message.delete", params).await
    }

    pub async fn reset_conversation_style(
        &self,
        params: ResetConversationStyleParams,
    ) -> Result<ResetConversationStyleResponse, VKontakteError> {
        self.api_client
            .api_request("messages.resetConversationStyle", params)
            .await
    }

    pub async fn set_chat_photo(
        &self,
        params: SetChatPhotoParams,
    ) -> Result<SetChatPhotoResponse, VKontakteError> {
        self.api_client
            .api_request("messages.setChatPhoto", params)
            .await
    }

    pub async fn set_conversation_style(
        &self,
        params: SetConversationStyleParams,
    ) -> Result<SetConversationStyleResponse, VKontakteError> {
        self.api_client
            .api_request("messages.setConversationStyle", params)
            .await
    }
}

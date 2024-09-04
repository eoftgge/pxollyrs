use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::messages::set_chat_photo::SetChatPhotoParams;
use crate::vkontakte::types::params::photos::get_chat_upload_server::GetChatUploadServerParams;
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde::Deserialize;
use crate::pxolly::types::events::event_type::EventType;

#[derive(Debug, Clone, Deserialize)]
struct UploadPhotoResponse {
    response: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatPhotoUpdateObject {
    chat_id: String,
    chat_local_id: Option<u64>,
    photo_url: String,
    is_remove: u8,
}

pub struct ChatPhotoUpdate {
    pub(crate) vkontakte: VKontakteAPI,
    pub(crate) http: Client,
}

impl Handler for ChatPhotoUpdate {
    const EVENT_TYPE: EventType = EventType::ChatPhotoUpdate;
    type EventObject = ChatPhotoUpdateObject;

    async fn handle(
        &self,
        object: Self::EventObject,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let chat_id = object.chat_local_id.ok_or_else(PxollyWebhookError::chat_not_found)?;
        let params = GetChatUploadServerParams { chat_id };
        let response = self
            .vkontakte
            .photos()
            .get_chat_upload_server(params)
            .await?;
        let photo = self
            .http
            .get(&object.photo_url)
            .send()
            .await?
            .bytes()
            .await?;
        let response = self
            .http
            .post(response.upload_url)
            .multipart(Form::new().part("file", Part::stream(photo).file_name("file1.png")))
            .send()
            .await?
            .json::<UploadPhotoResponse>()
            .await?
            .response;
        self.vkontakte
            .messages()
            .set_chat_photo(SetChatPhotoParams { file: response })
            .await?;

        Ok(PxollyWebhookResponse::new(true))
    }
}

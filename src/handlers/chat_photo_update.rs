use crate::pxolly::dispatch::handler::Handler;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::errors::PxollyWebhookError;
use crate::pxolly::types::responses::webhook::PxollyWebhookResponse;
use crate::vkontakte::api::VKontakteAPI;
use crate::vkontakte::types::categories::Categories;
use crate::vkontakte::types::params::messages::set_chat_photo::SetChatPhotoParams;
use crate::vkontakte::types::params::photos::get_chat_upload_server::GetChatUploadServerParams;
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::Value;

pub struct ChatPhotoUpdate {
    pub(crate) vkontakte: VKontakteAPI,
    pub(crate) http: Client,
}

impl Handler for ChatPhotoUpdate {
    const EVENT_TYPE: &'static str = "chat_photo_update";

    async fn handle(
        &self,
        event: PxollyEvent,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        let params = GetChatUploadServerParams {
            chat_id: event.object.chat_local_id.unwrap() as u64,
        };
        let response = self
            .vkontakte
            .photos()
            .get_chat_upload_server(params)
            .await?;
        let photo = self
            .http
            .get(
                event
                    .object
                    .photo_url
                    .as_ref()
                    .expect("Expected field `photo_url`"),
            )
            .send()
            .await?
            .bytes()
            .await?;
        let response = &self
            .http
            .post(response.upload_url)
            .multipart(Form::new().part("file", Part::stream(photo).file_name("file1.png")))
            .send()
            .await?
            .json::<Value>()
            .await?;
        let body = response["response"]
            .as_str()
            .ok_or_else(PxollyWebhookError::internal_server)?;
        self.vkontakte
            .messages()
            .set_chat_photo(SetChatPhotoParams { file: body.into() })
            .await?;

        Ok(PxollyWebhookResponse::new(true))
    }
}

use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::Value;

use crate::handlers::prelude::*;

pub struct ChatPhotoUpdate {
    api_client: APIClient,
    http_client: Client,
}

impl ChatPhotoUpdate {
    pub fn new(api_client: APIClient) -> Self {
        Self {
            api_client,
            http_client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl TraitHandler for ChatPhotoUpdate {
    const EVENT_TYPE: &'static str = "chat_photo_update";

    async fn execute(&self, ctx: PxollyContext) -> PxollyResult<PxollyResponse> {
        let response_url = self
            .api_client
            .api_request::<Value>(
                "photos.getChatUploadServer",
                par! {
                    "chat_id": ctx.peer_id()? - 2_000_000_000
                },
            )
            .await?;
        let photo = self
            .http_client
            .get(
                ctx.object
                    .photo_url
                    .as_ref()
                    .expect("Expected field `photo_url`"),
            )
            .send()
            .await?
            .bytes()
            .await?;
        let response = &self
            .http_client
            .post(response_url["upload_url"].as_str().unwrap())
            .multipart(Form::new().part("file", Part::stream(photo).file_name("file1.png")))
            .send()
            .await?
            .json::<Value>()
            .await?;
        let body = response["response"]
            .as_str()
            .ok_or_else(|| PxollyError::from("response isn't str...."))? as &str;
        let _ = self
            .api_client
            .api_request::<Value>(
                "messages.setChatPhoto",
                par! {
                    "file": body
                },
            )
            .await?;

        Ok(PxollyResponse::Success)
    }
}

mod chat_photo_update;
mod confirmation;
mod delete_for_all;
mod invite_user;
mod reset_theme;
mod set_theme;
mod sync;

pub mod prelude {
    pub use crate::errors::{WebhookError, WebhookResult};
    pub use crate::pxolly::dispatch::context::PxollyContext;
    pub use crate::pxolly::dispatch::handler::Handler;
    pub use crate::pxolly::types::responses::PxollyResponse;
    pub use crate::vk::client::VKClient;
}

use crate::pxolly::dispatch::dispatcher::{DispatcherBuilder, PushHandler};
use crate::pxolly::dispatch::execute::Dispatch;
use crate::vk::client::VKClient;
use reqwest::Client;
use std::sync::Arc;

pub fn build_dispatcher(
    vk_client: VKClient,
    http_client: Arc<Client>,
    confirmation_code: String,
) -> impl Dispatch {
    DispatcherBuilder
        .push_handler(chat_photo_update::ChatPhotoUpdate {
            vk_client: vk_client.clone(),
            http_client,
        })
        .push_handler(delete_for_all::DeleteForAll {
            vk_client: vk_client.clone(),
        })
        .push_handler(invite_user::InviteUser {
            vk_client: vk_client.clone(),
        })
        .push_handler(reset_theme::ResetTheme {
            vk_client: vk_client.clone(),
        })
        .push_handler(set_theme::SetTheme {
            vk_client: vk_client.clone(),
        })
        .push_handler(sync::Sync { vk_client })
        .push_handler(confirmation::Confirmation { confirmation_code })
}

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

use crate::pxolly::dispatch::dispatcher::{Dispatch, DispatcherBuilder};
use crate::vk::client::VKClient;
use reqwest::Client;
use std::sync::Arc;
use crate::pxolly::dispatch::compose::ComposeHandler;

pub fn build_dispatcher(
    vk_client: VKClient,
    http_client: Arc<Client>,
    confirmation_code: String,
) -> impl Dispatch {
    DispatcherBuilder
        .compose(chat_photo_update::ChatPhotoUpdate {
            vk_client: vk_client.clone(),
            http_client,
        })
        .compose(delete_for_all::DeleteForAll {
            vk_client: vk_client.clone(),
        })
        .compose(invite_user::InviteUser {
            vk_client: vk_client.clone(),
        })
        .compose(reset_theme::ResetTheme {
            vk_client: vk_client.clone(),
        })
        .compose(set_theme::SetTheme {
            vk_client: vk_client.clone(),
        })
        .compose(sync::Sync { vk_client })
        .compose(confirmation::Confirmation { confirmation_code })
}

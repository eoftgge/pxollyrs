mod chat_photo_update;
mod confirmation;
mod delete_for_all;
mod invite_user;
mod reset_theme;
mod set_theme;
mod sync;

use crate::pxolly::dispatch::compose::ComposeHandler;
use crate::pxolly::dispatch::dispatcher::{Dispatch, DispatcherBuilder};
use reqwest::Client;
use crate::vkontakte::api::VKontakteAPI;

pub fn build_dispatcher(
    vkontakte: VKontakteAPI,
    http: Client,
    confirmation_code: String,
) -> impl Dispatch {
    DispatcherBuilder
        .compose(chat_photo_update::ChatPhotoUpdate {
            vkontakte: vkontakte.clone(),
            http,
        })
        .compose(delete_for_all::DeleteForAll {
            vkontakte: vkontakte.clone(),
        })
        .compose(invite_user::InviteUser {
            vkontakte: vkontakte.clone(),
        })
        .compose(reset_theme::ResetTheme {
            vkontakte: vkontakte.clone(),
        })
        .compose(set_theme::SetTheme {
            vkontakte: vkontakte.clone(),
        })
        .compose(sync::Sync { vkontakte })
        .compose(confirmation::Confirmation { confirmation_code })
}

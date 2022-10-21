mod chat_members;
mod chat_photo_update;
mod confirmation;
mod delete_for_all;
mod events_get;
mod execute;
mod group_ban;
mod group_unban;
mod invite_user;
mod reset_theme;
mod set_admin;
mod set_theme;
mod sync;

pub mod prelude {
    pub use crate::errors::{WebhookError, WebhookResult};
    pub use crate::par;
    pub use crate::pxolly::dispatch::context::PxollyContext;
    pub use crate::pxolly::dispatch::traits::TraitHandler;
    pub use crate::pxolly::types::responses::PxollyResponse;
    pub use crate::vk::api::VKAPI;
}

use crate::database::conn::DatabaseConn;
use crate::pxolly::api::PxollyAPI;
use crate::pxolly::dispatch::dispatcher::{DispatcherBuilder, PushHandler, EVENT_TYPES_HANDLERS};
use crate::pxolly::dispatch::execute::Execute;
use crate::vk::api::VKAPI;

pub fn build_dispatcher(
    pxolly_client: PxollyAPI,
    api_client: VKAPI,
    conn: DatabaseConn,
) -> impl Execute {
    DispatcherBuilder
        .push_handler(chat_members::ChatMembers {
            api_client: api_client.clone(),
        })
        .push_handler(chat_photo_update::ChatPhotoUpdate::new(api_client.clone()))
        .push_handler(delete_for_all::DeleteForAll {
            api_client: api_client.clone(),
        })
        .push_handler(execute::Execute {
            api_client: api_client.clone(),
        })
        .push_handler(group_ban::GroupBan {
            api_client: api_client.clone(),
        })
        .push_handler(group_unban::GroupUnban {
            api_client: api_client.clone(),
        })
        .push_handler(invite_user::InviteUser {
            api_client: api_client.clone(),
        })
        .push_handler(reset_theme::ResetTheme {
            api_client: api_client.clone(),
        })
        .push_handler(set_admin::SetAdmin {
            api_client: api_client.clone(),
        })
        .push_handler(set_theme::SetTheme {
            api_client: api_client.clone(),
        })
        .push_handler(sync::Sync { api_client, conn })
        .push_handler(confirmation::Confirmation { pxolly_client }) // WARNING: IT'S ALWAYS PENULTIMATE
        .push_handler(events_get::EventsGet {
            handlers: unsafe { EVENT_TYPES_HANDLERS.clone() }, // IT SAFE CODE!!!! I SWEAR BY MY MOM!!!!
        }) // WARNING: IT'S ALWAYS LAST!!!
}

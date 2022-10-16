mod chat_members;
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
    pub use crate::api::client::APIClient;
    pub use crate::errors::{PxollyError, PxollyResult};
    pub use crate::par;
    pub use crate::pxolly::context::PxollyContext;
    pub use crate::pxolly::traits::TraitHandler;
    pub use crate::pxolly::types::responses::PxollyResponse;
}

use crate::api::client::APIClient;
use crate::pxolly::dispatcher::{DispatcherBuilder, PushHandler, EVENT_TYPES_HANDLERS};
use crate::pxolly::execute::Execute;
use crate::utils::database::DatabaseJSON;

pub fn build_dispatcher<'a>(
    confirmation_code: String,
    client: APIClient,
    database: &DatabaseJSON,
) -> impl Execute {
    DispatcherBuilder
        .push_handler(chat_members::ChatMembers {
            client: client.clone(),
        })
        .push_handler(delete_for_all::DeleteForAll {
            client: client.clone(),
        })
        .push_handler(execute::Execute {
            client: client.clone(),
        })
        .push_handler(group_ban::GroupBan {
            client: client.clone(),
        })
        .push_handler(group_unban::GroupUnban {
            client: client.clone(),
        })
        .push_handler(invite_user::InviteUser {
            client: client.clone(),
        })
        .push_handler(reset_theme::ResetTheme {
            client: client.clone(),
        })
        .push_handler(set_admin::SetAdmin {
            client: client.clone(),
        })
        .push_handler(set_theme::SetTheme {
            client: client.clone(),
        })
        .push_handler(sync::Sync {
            client: client,
            database: database.clone(),
        })
        .push_handler(confirmation::Confirmation { confirmation_code }) // WARNING: IT'S ALWAYS PENULTIMATE
        .push_handler(events_get::EventsGet {
            handlers: unsafe { EVENT_TYPES_HANDLERS.clone() },
        }) // WARNING: IT'S ALWAYS LAST!!!
}

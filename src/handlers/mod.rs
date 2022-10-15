mod chat_members;
mod confirmation;
mod delete_for_all;
mod sync;

use crate::api::client::APIClient;
use crate::pxolly::dispatcher::{DispatcherBuilder, PushHandler};
use crate::pxolly::execute::Execute;
use crate::utils::database::DatabaseJSON;
use std::sync::Arc;

pub fn build_dispatcher(
    confirmation_code: String,
    client: APIClient,
    database: DatabaseJSON,
) -> impl Execute {
    let database = Arc::new(database);
    DispatcherBuilder
        .push_handler(chat_members::ChatMembers {
            client: client.clone(),
        })
        .push_handler(delete_for_all::DeleteForAll {
            client: client.clone(),
        })
        .push_handler(sync::Sync {
            client,
            database: Arc::clone(&database),
        })
        .push_handler(confirmation::Confirmation { confirmation_code }) // WARNING: IT'S ALWAYS LAST!!!
}

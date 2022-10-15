mod chat_members;
mod delete_for_all;
mod sync;

use crate::pxolly::dispatcher::{DispatcherBuilder, PushHandler};
use crate::pxolly::execute::Execute;

pub fn build_dispatcher() -> impl Execute {
    DispatcherBuilder
        .push_handler(sync::Sync)
        .push_handler(chat_members::ChatMembers)
        .push_handler(delete_for_all::DeleteForAll)
}

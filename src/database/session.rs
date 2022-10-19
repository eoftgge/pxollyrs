use crate::database::models::PxollyChatModel;

pub struct DatabaseSession {
    cache: Vec<PxollyChatModel>,
}

impl DatabaseSession {
    pub(super) fn new(cache: Vec<PxollyChatModel>) -> Self {
        Self { cache }
    }
}

use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ImportChatLocalIdsParams {
    pub(crate) chat_local_ids: HashMap<String, u64>,
}
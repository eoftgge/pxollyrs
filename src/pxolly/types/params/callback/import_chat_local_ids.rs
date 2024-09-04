use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct ImportChatLocalIdsParams {
    pub(crate) chat_local_ids: HashMap<String, u64>,
}

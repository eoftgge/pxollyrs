use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessagesDeleteParams {
    pub(crate) peer_id: i64,
    pub(crate) delete_for_all: u8,
    pub(crate) cmids: Vec<u64>,
}
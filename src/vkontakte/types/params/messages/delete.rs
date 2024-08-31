use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessagesDeleteParams {
    peer_id: i64,
    delete_for_all: u8,
    cmids: Vec<u64>,
}
use serde::Deserialize;
use crate::pxolly::types::events::event_type::EventType;

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyEvent {
    pub event_type: EventType,
    pub secret_key: String,
    pub event_id: String,
    pub from_id: Option<i64>,
    pub user_id: Option<i64>,
    pub object: serde_value::Value,
}
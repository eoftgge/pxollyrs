use crate::pxolly::types::events::event_type::EventType;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyEvent {
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub secret_key: String,
    pub event_id: String,
    pub from_id: Option<i64>,
    pub user_id: Option<i64>,
    pub object: serde_value::Value,
}

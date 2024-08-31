pub mod object;
pub mod user;
pub mod message;

use serde::Deserialize;
use crate::pxolly::types::events::object::PxollyObject;

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub secret_key: String,
    pub event_id: String,
    pub from_id: Option<i64>,
    pub user_id: Option<i64>,
    pub object: PxollyObject,
}

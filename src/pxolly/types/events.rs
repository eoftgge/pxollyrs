pub mod message;
pub mod object;
pub mod user;

use crate::pxolly::types::events::object::PxollyObject;
use serde::Deserialize;

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

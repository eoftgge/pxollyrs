pub mod edit_settings;
pub mod get_settings;

pub use edit_settings::EditSettingsResponse;
pub use get_settings::GetSettingsResponse;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PxollyAPIResponse<T> {
    Response(T),
    Error(PxollyAPIError),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PxollyAPIError {
    pub error_code: i32,
    pub error_msg: String,
    pub request_params: Vec<PxollyRequestParams>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PxollyRequestParams {
    pub key: String,
    pub value: String,
}


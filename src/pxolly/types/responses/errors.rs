use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::handlers::prelude::PxollyResponse;
use crate::WebhookError;

#[derive(Debug, Copy, Clone)]
#[repr(i8)]
pub enum PxollyErrorCode {
    VKAPI = -1,
    Unknown = 2,
    IO = 3,
}

impl From<PxollyErrorCode> for i8 {
    fn from(value: PxollyErrorCode) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Display for PxollyErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let integer: i8 = (*self).into();
        let string = integer.to_string();
        write!(f, "{}", string)
    }
}

impl From<WebhookError> for PxollyResponse {
    fn from(value: WebhookError) -> Self {
        let error_code = match value {
            WebhookError::VKAPI(err) => {
                log::error!("in the dispatcher occurred api error: {:?}", err);
                PxollyErrorCode::VKAPI
            }
            WebhookError::IO(err) => {
                log::error!("in the dispatcher occurred io error: {:?}", err);
                PxollyErrorCode::IO
            }
            err => {
                log::error!("in the dispatcher occurred unknown error: {:?}", err);
                PxollyErrorCode::Unknown
            }
        };
        
        Self::ErrorCode(error_code)
    }
}
pub mod edit_settings;
pub mod get_settings;

use axum::response::{IntoResponse, Response};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PxollyResponse {
    Text(String),
    ConfirmationCode(String),
    ErrorCode(i8),
    Success,
    Locked,
}

impl Display for PxollyResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Text(text) => text.into(),
            Self::ConfirmationCode(code) => code.into(),
            Self::ErrorCode(code) => code.to_string(),
            Self::Success => "1".into(),
            Self::Locked => "locked".into(),
        };
        write!(f, "{}", s)
    }
}

impl IntoResponse for PxollyResponse {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}

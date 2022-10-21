pub mod edit_settings;
pub mod get_settings;

use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum PxollyResponse {
    Text(String),
    ConfirmationCode(String),
    ErrorCode(i8),
    Success,
    Locked,
}

impl ToString for PxollyResponse {
    fn to_string(&self) -> String {
        match self {
            Self::Text(text) => text.into(),
            Self::ConfirmationCode(code) => code.into(),
            Self::ErrorCode(code) => code.to_string(),
            Self::Success => "1".into(),
            Self::Locked => "locked".into(),
        }
    }
}

impl IntoResponse for PxollyResponse {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}

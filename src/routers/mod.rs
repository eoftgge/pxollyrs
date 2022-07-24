use crate::errors::PxollyError;
use crate::routers::handler::PxollyHandler;
use crate::utils::models::{PxollyEvent, PxollyResponse};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;

pub mod handler;

pub async fn handle(
    Json(event): Json<PxollyEvent>,
    Extension(p_handler): Extension<PxollyHandler>,
) -> Result<String, (StatusCode, String)> {
    match p_handler.handle(event).await {
        Ok(response) => Ok(response.to_string()),
        Err(error) => {
            log::error!("An unexpected error occurred: {:?}", error);

            if let PxollyError::API(_) = error {
                Ok(PxollyResponse::ErrorCode(-4).to_string())
            } else if let PxollyError::Response(response) = error {
                Ok(response.to_string())
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                ))
            }
        }
    }
}

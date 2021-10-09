use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;
use crate::routers::handler::PxollyHandler;
use crate::utils::models::{PxollyEvent, PxollyResponse};
use crate::errors::PxollyError;

pub mod handler;

type ResultHandle = Result<String, (StatusCode, String)>;

pub async fn handle(Json(event): Json<PxollyEvent>, Extension(p_handler): Extension<PxollyHandler>) -> ResultHandle {
	match p_handler.handle(event).await {
		Ok(response) => Ok(response.to_string()),
		Err(error) => {
			log::error!("An unexpected error occurred: {:?}", error);

			return if let PxollyError::API(_) = error {
				Ok(PxollyResponse::Fail.to_string())
			} else {
				Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()))
			}
		}
	}
}

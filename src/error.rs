use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub struct AnyHowError {
    err: anyhow::Error,
}

impl actix_web::error::ResponseError for AnyHowError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .content_type("text/plain")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<anyhow::Error> for AnyHowError {
    fn from(err: anyhow::Error) -> Self {
        Self { err }
    }
}

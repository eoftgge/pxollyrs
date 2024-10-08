use crate::pxolly::dispatch::dispatcher::Dispatch;
use crate::pxolly::types::events::event::PxollyEvent;
use crate::pxolly::types::responses::errors::{PxollyErrorType, PxollyWebhookError};
use crate::pxolly::types::responses::webhook::{PxollyWebhookResponse, WebhookResponse};
use axum::body::Body;
use axum::extract::FromRequest;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct Executor<T: Dispatch> {
    dispatcher: Arc<T>,
    secret_key: String,
}

impl<T: Dispatch> Executor<T> {
    pub fn new(dispatcher: T, secret_key: impl Into<String>) -> Self {
        Self {
            dispatcher: Arc::new(dispatcher),
            secret_key: secret_key.into(),
        }
    }

    async fn execute(
        &self,
        Json(event): Json<PxollyEvent>,
    ) -> Result<PxollyWebhookResponse, PxollyWebhookError> {
        log::debug!("received the event: {:?}", event);

        if event.secret_key != self.secret_key {
            return Err(PxollyWebhookError {
                message: None,
                error_type: PxollyErrorType::AccessDenied,
            });
        }

        let response = self.dispatcher.dispatch(event).await;
        log::debug!("response to the server: {:?}", response);
        response
    }
}

impl<T: Dispatch> Clone for Executor<T> {
    fn clone(&self) -> Self {
        Self {
            dispatcher: Arc::clone(&self.dispatcher),
            secret_key: self.secret_key.clone(),
        }
    }
}

impl<E: Dispatch, S: Send + Sync + 'static> axum::handler::Handler<(), S> for Executor<E> {
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: Request<Body>, state: S) -> Self::Future {
        Box::pin(async move {
            log::debug!("The raw request: {:?}", req);
            let event = match Json::from_request(req, &state).await {
                Ok(event) => event,
                Err(err) => return err.into_response(),
            };
            let result = self.execute(event).await;
            let response = match result {
                Ok(response) => WebhookResponse::Ok(response),
                Err(err) => WebhookResponse::Error(err),
            };
            response.into_response()
        })
    }
}

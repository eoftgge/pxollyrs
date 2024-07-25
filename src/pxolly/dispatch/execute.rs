use super::context::PxollyContext;
use super::dispatcher::Dispatch;
use crate::database::conn::DatabaseConnection;
use crate::errors::WebhookError;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::PxollyResponse;
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
    database: DatabaseConnection,
}

impl<T: Dispatch> Executor<T> {
    pub fn new(dispatcher: T, database: DatabaseConnection, secret_key: impl Into<String>) -> Self {
        Self {
            dispatcher: Arc::new(dispatcher),
            secret_key: secret_key.into(),
            database,
        }
    }

    async fn execute(&self, Json(event): Json<PxollyEvent>) -> PxollyResponse {
        log::debug!("received the event: {:?}", event);

        if event.secret_key != self.secret_key {
            return PxollyResponse::Locked;
        }

        let ctx = PxollyContext::new(event, self.database.clone());
        let response = match self.dispatcher.dispatch(ctx).await {
            Ok(response) => response,
            Err(WebhookError::VKAPI(err)) => {
                log::error!("in the dispatcher occurred api error: {:?}", err);
                PxollyResponse::ErrorCode(-1)
            }
            Err(WebhookError::PxollyResponse(response)) => response,
            Err(WebhookError::IO(err)) => {
                log::error!("in the dispatcher occurred io error: {:?}", err);
                PxollyResponse::ErrorCode(3)
            }
            Err(err) => {
                log::error!("in the dispatcher occurred unknown error: {:?}", err);
                PxollyResponse::ErrorCode(2)
            }
        };

        log::debug!("response to the sender: {}", response.to_string());
        response
    }
}

impl<T: Dispatch> Clone for Executor<T> {
    fn clone(&self) -> Self {
        Self {
            dispatcher: Arc::clone(&self.dispatcher),
            database: self.database.clone(),
            secret_key: self.secret_key.clone(),
        }
    }
}

impl<E: Dispatch, S: Send + Sync + 'static> axum::handler::Handler<(), S> for Executor<E> {
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: Request<Body>, state: S) -> Self::Future {
        Box::pin(async move {
            self.execute(match Json::from_request(req, &state).await {
                Ok(event) => event,
                Err(err) => return err.into_response(),
            })
            .await
            .into_response()
        })
    }
}

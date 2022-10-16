use super::context::PxollyContext;
use super::dispatcher::{Dispatcher, DispatcherBuilder};
use super::traits::TraitHandler;
use super::types::events::PxollyEvent;
use super::types::responses::PxollyResponse;
use crate::errors::{PxollyError, PxollyResult};
use crate::utils::database::DatabaseJSON;
use axum::body::Body;
use axum::extract::{FromRequest, RequestParts};
use axum::handler::Handler;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Execute: Send + Sync + Clone {
    async fn execute(&self, ctx: PxollyContext) -> PxollyResult<PxollyResponse>;
}

#[async_trait::async_trait]
impl Execute for DispatcherBuilder {
    async fn execute(&self, _: PxollyContext) -> PxollyResult<PxollyResponse> {
        Ok(PxollyResponse::ErrorCode(0))
    }
}

#[async_trait::async_trait]
impl<Handler, Tail> Execute for Dispatcher<Handler, Tail>
where
    Handler: TraitHandler,
    Tail: Execute + Send + Sync + 'static,
{
    async fn execute(&self, ctx: PxollyContext) -> PxollyResult<PxollyResponse> {
        if Handler::EVENT_TYPE == ctx.event_type {
            return self.handler.execute(ctx).await;
        }
        self.tail.execute(ctx).await
    }
}

#[derive(Clone)]
pub struct Executor<E: Execute> {
    executor: Arc<E>,
    secret_key: String,
    database: DatabaseJSON,
}

impl<E: Execute> Executor<E> {
    pub fn new(executor: E, secret_key: String, database: DatabaseJSON) -> Self {
        Self {
            executor: Arc::new(executor),
            secret_key,
            database,
        }
    }

    async fn execute(&self, Json(event): Json<PxollyEvent>) -> PxollyResponse {
        log::debug!("received the event: {:?}", event);

        if event.secret_key != self.secret_key {
            return PxollyResponse::Locked;
        }

        let peer_id = match event.object.chat_id.as_ref() {
            Some(chat_id) => self.database.get(chat_id).await,
            _ => None,
        };
        let ctx = PxollyContext::new(event, peer_id);
        let response = match self.executor.execute(ctx).await {
            Ok(response) => response,
            Err(PxollyError::API(_)) => PxollyResponse::ErrorCode(-4),
            Err(PxollyError::Response(response)) => response,
            Err(PxollyError::IO(_)) => PxollyResponse::ErrorCode(3),
            Err(err) => {
                log::error!("in the dispatcher occurred unknown error: {:?}", err);
                PxollyResponse::ErrorCode(0)
            }
        };

        log::debug!("response to the sender: {}", response.to_string());
        response
    }
}

impl<E: Execute + 'static> Handler<()> for Executor<E> {
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: Request<Body>) -> Self::Future {
        let mut req = RequestParts::new(req);
        Box::pin(async move {
            self.execute(match Json::<PxollyEvent>::from_request(&mut req).await {
                Ok(event) => event,
                Err(err) => return err.into_response(),
            })
            .await
            .into_response()
        })
    }
}

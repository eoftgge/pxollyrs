use super::context::PxollyContext;
use super::dispatcher::{Dispatcher, DispatcherBuilder};
use super::handler::Handler;
use crate::database::conn::DatabaseConn;
use crate::errors::{WebhookError, WebhookResult};
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::PxollyResponse;
use axum::body::Body;
use axum::extract::{FromRequest};
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use convert_case::{Case, Casing};

#[async_trait::async_trait]
pub trait Dispatch: Send + Sync + Clone {
    async fn dispatch(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse>;
}

#[async_trait::async_trait]
impl Dispatch for DispatcherBuilder {
    async fn dispatch(&self, _: PxollyContext) -> WebhookResult<PxollyResponse> {
        Ok(PxollyResponse::ErrorCode(0))
    }
}

#[async_trait::async_trait]
impl<H, Tail> Dispatch for Dispatcher<H, Tail>
where
    H: Handler,
    Tail: Dispatch + Send + Sync + 'static,
{
    async fn dispatch(&self, ctx: PxollyContext) -> WebhookResult<PxollyResponse> {
        let name_handler = stringify!(H).to_case(Case::Snake);
        if name_handler == ctx.event_type {
            return self.handler.handle(ctx).await;
        }
        self.tail.dispatch(ctx).await
    }
}

#[derive(Clone)]
pub struct Executor<D: Dispatch> {
    dispatcher: Arc<D>,
    secret_key: String,
    conn: DatabaseConn,
}

impl<D: Dispatch> Executor<D> {
    pub fn new(dispatcher: D, conn: DatabaseConn, secret_key: impl Into<String>) -> Self {
        Self {
            dispatcher: Arc::new(dispatcher),
            secret_key: secret_key.into(),
            conn,
        }
    }

    async fn execute(&self, Json(event): Json<PxollyEvent>) -> PxollyResponse {
        log::debug!("received the event: {:?}", event);

        if event.secret_key != self.secret_key {
            return PxollyResponse::Locked;
        }

        let ctx = PxollyContext::new(event, self.conn.clone());
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

impl<E: Dispatch + 'static, S: Send + Sync + 'static> axum::handler::Handler<(), S> for Executor<E> {
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

use super::dispatcher::{Dispatcher, DispatcherBuilder};
use super::traits::TraitHandler;
use super::types::events::PxollyEvent;
use super::types::responses::PxollyResponse;
use crate::api::client::APIClient;
use crate::errors::PxollyError;
use crate::pxolly::context::HandlerContext;
use crate::utils::config::SecretKey;
use crate::utils::database::DatabaseJSON;
use crate::utils::ConfirmationCode;
use crate::PxollyResult;
use axum::{Extension, Json};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Execute: Send + Sync {
    async fn execute(&self, ctx: HandlerContext) -> PxollyResult<PxollyResponse>;
}

#[async_trait::async_trait]
impl Execute for DispatcherBuilder {
    async fn execute(&self, _: HandlerContext) -> PxollyResult<PxollyResponse> {
        Ok(PxollyResponse::ErrorCode(0))
    }
}

#[async_trait::async_trait]
impl<Handler, Tail> Execute for Dispatcher<Handler, Tail>
where
    Handler: TraitHandler,
    Tail: Execute + Send + Sync + 'static,
{
    async fn execute(&self, ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
        if Handler::EVENT_TYPE == ctx.event_type {
            return self.handler.execute(ctx).await;
        }
        self.tail.execute(ctx).await
    }
}

fn handle_error(err: PxollyError) -> PxollyResponse {
    match err {
        PxollyError::API(_) => PxollyResponse::ErrorCode(-4),
        PxollyError::Response(response) => response,
        PxollyError::IO(_) => PxollyResponse::ErrorCode(3),
        _ => PxollyResponse::ErrorCode(0),
    }
}

pub async fn handle(
    Json(event): Json<PxollyEvent>,
    Extension(dp): Extension<Arc<dyn Execute>>,
    Extension(database): Extension<Arc<DatabaseJSON>>,
    Extension(code): Extension<ConfirmationCode>,
    Extension(client): Extension<APIClient>,
    Extension(key): Extension<SecretKey>,
) -> String {
    log::debug!("Received the new event: {:?}", event);

    let peer_id = match event.object.chat_id.as_ref() {
        None if event.secret_key != *code.0 => return "locked".into(),
        None if event.event_type == "confirmation" => return "0".into(),
        None => return "2".into(),
        Some(_) if event.event_type == "sync" => None,
        Some(chat_id) => database.get(chat_id).await,
    };
    let ctx = HandlerContext {
        event,
        database,
        code,
        key,
        client,
        peer_id,
    };
    let response = match dp.execute(ctx).await {
        Ok(response) => response,
        Err(err) => handle_error(err),
    };

    log::debug!("Response to the sender: {}", response.to_string());

    response.to_string()
}

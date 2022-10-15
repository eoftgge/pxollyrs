use super::dispatcher::{Dispatcher, DispatcherBuilder};
use super::traits::TraitHandler;
use super::types::events::PxollyEvent;
use super::types::responses::PxollyResponse;
use crate::errors::PxollyError;
use crate::pxolly::context::HandlerContext;
use crate::utils::database::DatabaseJSON;
use crate::PxollyResult;
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

fn handle_errors(err: PxollyError) -> PxollyResponse {
    match err {
        PxollyError::API(_) => PxollyResponse::ErrorCode(-4),
        PxollyError::Response(response) => response,
        PxollyError::IO(_) => PxollyResponse::ErrorCode(3),
        _ => PxollyResponse::ErrorCode(0),
    }
}

pub async fn handle(
    event: PxollyEvent,
    secret_key: &str,
    database: &DatabaseJSON,
    dispatcher: Arc<impl Execute>,
) -> PxollyResponse {
    log::debug!("received the new event: {:?}", event);

    if event.secret_key != secret_key {
        return PxollyResponse::Locked;
    }

    let response = match dispatcher
        .execute(HandlerContext {
            peer_id: match event.object.chat_id.as_ref() {
                Some(chat_id) => database.get(chat_id).await,
                _ => None,
            },
            event,
        })
        .await
    {
        Ok(response) => response,
        Err(err) => handle_errors(err),
    };

    log::debug!("response to the sender: {}", response.to_string());

    response
}

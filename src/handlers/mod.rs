use crate::api::client::APIClient;
use crate::errors::PxollyError;
use crate::utils::config::SecretKey;
use crate::utils::database::DatabaseJSON;
use crate::utils::models::{PxollyEvent, PxollyResponse};
use crate::utils::ConfirmationCode;
use crate::PxollyResult;
use axum::extract::Extension;
use axum::Json;
use std::sync::Arc;

mod chat_members;
mod delete_for_all;
mod events_get;
mod execute;
mod group_ban;
mod group_unban;
mod invite_user;
mod reset_theme;
mod set_admin;
mod set_theme;
mod sync;

pub const HANDLERS: &[&str] = &[
    "events_get",
    "group_unban",
    "group_ban",
    "delete_for_all",
    "set_admin",
    "set_theme",
    "reset_theme",
    "sync",
    "confirmation",
];

pub struct HandlerContext {
    pub database: Arc<DatabaseJSON>, // TODO: maybe change db to other?
    pub code: ConfirmationCode,
    pub key: SecretKey,
    pub client: APIClient,
    pub event: PxollyEvent,
    pub peer_id: u64,
}

impl std::ops::Deref for HandlerContext {
    type Target = PxollyEvent;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

async fn execute_handler(mut ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    ctx.peer_id = match ctx.object.chat_id.as_ref() {
        None if ctx.secret_key != *ctx.key.0 => return Ok(PxollyResponse::Locked),
        None if ctx.c_type == "confirmation" => {
            return Ok(PxollyResponse::ConfirmationCode(ctx.code.0.to_string()))
        }
        None => return Ok(PxollyResponse::ErrorCode(-2)),
        Some(_) if ctx.c_type == "sync" => return sync::execute(ctx).await,
        Some(chat_id) => ctx
            .database
            .get(chat_id)
            .await
            .expect("Expect field: chat_id"),
    };

    let result = match &*ctx.c_type {
        "delete_for_all" => delete_for_all::execute(ctx).await?,
        "execute" => execute::execute(ctx).await?,
        "events_get" => events_get::execute().await?,
        "chat_members" => chat_members::execute(ctx).await?,
        "invite_user" => invite_user::execute(ctx).await?,
        "group_ban" => group_ban::execute(ctx).await?,
        "group_unban" => group_unban::execute(ctx).await?,
        "set_theme" => set_theme::execute(ctx).await?,
        "reset_theme" => reset_theme::execute(ctx).await?,
        "set_admin" => set_admin::execute(ctx).await?,
        "chat_photo_update" => PxollyResponse::ErrorCode(-3),
        unknown => {
            log::error!("Unknown type: {} // please, send a message to me about it error (vk - id260116872)", unknown);
            PxollyResponse::ErrorCode(0)
        }
    };

    Ok(result)
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
    Extension(database): Extension<Arc<DatabaseJSON>>,
    Extension(code): Extension<ConfirmationCode>,
    Extension(client): Extension<APIClient>,
    Extension(key): Extension<SecretKey>,
) -> String {
    log::debug!("Received the new event: {:?}", event);

    let ctx = HandlerContext {
        event,
        database,
        code,
        key,
        client,
        peer_id: u64::default(),
    };
    let response = match execute_handler(ctx).await {
        Ok(response) => response,
        Err(err) => handle_error(err),
    };

    log::debug!("Response to the sender: {}", response.to_string());

    response.to_string()
}

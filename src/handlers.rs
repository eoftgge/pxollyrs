use crate::error::AnyHowError;
use crate::events::PxollyResponse;
use crate::params;
use crate::tools::{parse_pxolly_response, vec_u64_to_string, Event};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use anyhow::anyhow;
use std::ops::Deref;

pub async fn handle_event(ev: Event) -> anyhow::Result<PxollyResponse> {
    let chat_id: u64;
    let result = ev
        .chat
        .get(
            ev.event
                .object
                .chat_id
                .as_ref()
                .ok_or(anyhow!("NoneError..."))?,
        )
        .await;

    if result.is_some() {
        chat_id = result.ok_or(anyhow!("NoneError..."))?;
    } else {
        return Ok(PxollyResponse::UnknownUIDOrNoSupport);
    }

    return Ok(match &*ev.event.ctype {
        "delete_for_all" => {
            let result = ev.api_ctx.api_request("messages.delete", &mut params! {
                    "delete_for_all" => "1".to_string(),
                    "peer_id" => (2000000000 + chat_id).to_string(),
                    "spam" => ev.event.object.is_spam.unwrap().to_string(),
                    "conversation_message_ids" => vec_u64_to_string(ev.event.object.conversation_message_ids.as_ref().unwrap())
                }).await;

            if result.is_err() {
                return Ok(PxollyResponse::UnknownUIDOrNoSupport);
            }

            PxollyResponse::Success
        }
        "invite_user" => {
            let result = ev.api_ctx.api_request("messages.addChatUser", &mut params! {
                    "visible_messages_count" => ev.event.object.visible_messages_count.unwrap_or(0).to_string(),
                    "user_id" => ev.event.object.user_id.expect("error!").to_string(),
                    "chat_id" => chat_id.to_string(),
                }).await;

            if result.is_err() {
                return Ok(PxollyResponse::UnknownErrorOrError);
            }

            PxollyResponse::Success
        }
        "group_ban" => {
            let result = ev
                .api_ctx
                .api_request(
                    "groups.ban",
                    &mut params! {
                        "end_date" => ev.event.object.expired.expect("error!").to_string(),
                        "group_id" => ev.event.object.group_id.expect("error!").to_string(),
                        "owner_id" => ev.event.object.user_id.expect("error!").to_string()
                    },
                )
                .await;

            if result.is_err() {
                return Ok(PxollyResponse::Fail);
            }

            PxollyResponse::Success
        }
        "group_unban" => {
            let result = ev
                .api_ctx
                .api_request(
                    "groups.unban",
                    &mut params! {
                        "group_id" => ev.event.object.group_id.expect("error!").to_string(),
                        "owner_id" => ev.event.object.user_id.expect("error!").to_string()
                    },
                )
                .await;

            if result.is_err() {
                return Ok(PxollyResponse::Fail);
            }

            PxollyResponse::Success
        }
        _ => {
            log::info!("Unknown event type...");
            PxollyResponse::UnknownErrorOrError
        }
    });
}

pub async fn index(
    event: web::Json<crate::events::PxollyEvent>,
    api_ctx: web::Data<crate::APIClient>,
    settings: web::Data<crate::settings::Settings>,
    chat: web::Data<crate::chat_data::WorkChatData>,
) -> Result<HttpResponse, AnyHowError> {
    if event.secret_key != settings.secret_key {
        return Ok(HttpResponse::build(StatusCode::LOCKED).body("locked"));
    }

    let mut response = HttpResponse::build(StatusCode::OK);

    return match &*event.ctype {
        "add_chat" => {
            chat.insert(
                event
                    .object
                    .chat_id
                    .as_ref()
                    .ok_or(anyhow!("NoneError..."))?
                    .to_string(),
                event.object.chat_uid.ok_or(anyhow!("NoneError..."))?,
            )
            .await?;
            Ok(response.content_type("text/plain").body("1"))
        }
        "confirmation" => Ok(response
            .content_type("text/plain")
            .body(&crate::tools::confirmation_code(settings.pxolly_token.as_ref()).await?)),
        _ => {
            let event = Event {
                chat: chat.deref().deref().clone(),
                event: event.deref().deref().clone(),
                api_ctx: api_ctx.deref().deref().deref().clone(),
            };
            Ok(response
                .content_type("text/plain")
                .body(parse_pxolly_response(handle_event(event).await?)))
        }
    };
}

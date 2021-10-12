use crate::api::client::APIClient;
use crate::errors::PxollyError;
use crate::par;
use crate::utils::models::{PxollyEvent, PxollyResponse};
use crate::utils::{database::DatabaseJSON, PxollyTools};
use crate::PxollyResult;

type ResultHandle = PxollyResult<PxollyResponse>;

#[derive(Clone)]
pub struct PxollyHandler {
    pub tools: PxollyTools,
    pub database: DatabaseJSON,
    pub api_client: APIClient,
}

impl PxollyHandler {
    async fn add_chat(&self, event: PxollyEvent) -> ResultHandle {
        log::info!("Add info chat's to db.");

        let chat_id = event.object.chat_id.ok_or(PxollyError::None)?;
        let chat_uid = event.object.chat_uid.ok_or(PxollyError::None)?;

        self.database.insert(chat_id, chat_uid).await?;
        Ok(PxollyResponse::Success)
    }

    async fn invite_user(&self, event: PxollyEvent, chat_id: u64) -> ResultHandle {
        let params = par! {
            "visible_messages_count": event.object.visible_messages_count.unwrap_or(0),
            "user_id": event.object.user_id
                .ok_or(PxollyError::None)?,
            "chat_id": chat_id
        };

        self.api_client
            .api_request("messages.addChatUser", params)
            .await?;
        Ok(PxollyResponse::Success)
    }

    async fn group_ban(&self, event: PxollyEvent) -> ResultHandle {
        let params = par! {
            "end_date": event.object.expired
                .ok_or(PxollyError::None)?,
            "group_id": event.object.group_id
                .ok_or(PxollyError::None)?,
            "owner_id": event.object.user_id
                .ok_or(PxollyError::None)?
        };

        self.api_client.api_request("groups.ban", params).await?;
        Ok(PxollyResponse::Success)
    }

    async fn group_unban(&self, event: PxollyEvent) -> ResultHandle {
        let params = par! {
            "group_id": event.object.group_id
                .ok_or(PxollyError::None)?,
            "owner_id": event.object.user_id
                .ok_or(PxollyError::None)?
        };

        self.api_client.api_request("groups.unban", params).await?;
        Ok(PxollyResponse::Success)
    }

    // ide...
    async fn dont_warn_set_theme(&self, event: PxollyEvent, chat_id: u64) -> ResultHandle {
        let params = par! {
            "peer_id": self.tools.get_peer_id(chat_id),
            "style": event.object.style
                .ok_or(PxollyError::None)?
        };

        self.api_client
            .api_request("messages.setConversationStyle", params)
            .await?;
        Ok(PxollyResponse::Success)
    }

    async fn reset_theme(&self, chat_id: u64) -> ResultHandle {
        let params = par! {
            "peer_id": self.tools.get_peer_id(chat_id)
        };

        self.api_client
            .api_request("messages.resetConversationStyle", params)
            .await?;
        Ok(PxollyResponse::Success)
    }

    pub async fn handle(&self, event: PxollyEvent) -> ResultHandle {
        let chat_id = match event.object.chat_id.as_ref() {
            None if event.secret_key != self.tools.config.secret_key => {
                return Ok(PxollyResponse::Locked)
            }
            None if event.c_type == "confirmation" => {
                return Ok(PxollyResponse::ConfirmationCode(
                    self.tools.get_confirmation_code(),
                ))
            }
            None => return Ok(PxollyResponse::UnknownUIDOrNoSupport),
            Some(_) if event.c_type == "add_chat" => return self.add_chat(event).await,
            Some(chat_id) => self
                .database
                .get(&*chat_id)
                .await
                .ok_or(PxollyError::None)?,
        };

        let result = match &*event.c_type {
            "invite_user" => self.invite_user(event, chat_id).await?,
            "group_ban" => self.group_ban(event).await?,
            "group_unban" => self.group_unban(event).await?,
            "set_theme" => self.dont_warn_set_theme(event, chat_id).await?,
            "reset_theme" => self.reset_theme(chat_id).await?,
            // set_admin
            unknown => {
                log::info!("Unknown type: {}", unknown);
                PxollyResponse::Fail
            }
        };

        Ok(result)
    }
}

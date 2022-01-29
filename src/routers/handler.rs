use crate::api::client::APIClient;
use crate::api::response::APIObjectResponse;
use crate::errors::PxollyError;
use crate::par;
use crate::utils::models::{PxollyEvent, PxollyResponse};
use crate::utils::{database::DatabaseJSON, PxollyTools};
use crate::PxollyResult;

type ResultHandle = PxollyResult<PxollyResponse>;

const CODE: &str = r#"
var h = API.messages.search({
    q: Args.text,
    count: 5
}).items;

var i = 0;
while(i < h.length) {
    if(Args.text == h[i].text && Args.from_id == h[i].from_id && Args.date == h[i].date && Args.conversation_message_id == h[i].conversation_message_id) {
        return h[i].peer_id;
    }
    i = i+1;
}
return false;
"#;

#[derive(Clone)]
pub struct PxollyHandler {
    pub tools: PxollyTools,
    pub database: DatabaseJSON,
    pub api_client: APIClient,
}

impl PxollyHandler {
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

    async fn set_admin(&self, event: PxollyEvent, chat_id: u64) -> ResultHandle {
        let params = par! {
            "peer_id": chat_id,
            "role": if event.object.admin.ok_or(PxollyError::None)? == 1 { "admin" } else { "member" },
            "user_id": event.object.user_id.ok_or(PxollyError::None)?,
        };

        self.api_client
            .api_request("messages.setRole", params)
            .await?;
        Ok(PxollyResponse::Success)
    }

    async fn sync(&self, event: PxollyEvent) -> ResultHandle {
        let message = event.object.message.ok_or(PxollyError::None)?;
        let params = par! {
            "code": CODE,
            "conversation_message_id": message.conversation_message_id,
            "text": message.text,
            "date": message.date,
            "from_id": message.from_id
        };
        let chat_id = event.object.chat_id.ok_or(PxollyError::None)?;

        if let APIObjectResponse::Integer(peer_id) =
            self.api_client.api_request("execute", params).await?
        {
            self.database
                .insert(chat_id, peer_id as u64)
                .await
                .map_err(|_| PxollyError::Response(PxollyResponse::FailDatabase))?;

            return Ok(PxollyResponse::ConfirmationCode(
                event.object.success.ok_or(PxollyError::None)?,
            ));
        }

        Ok(PxollyResponse::Fail)
    }

    pub async fn handle(&self, event: PxollyEvent) -> ResultHandle {
        let chat_id = match event.object.chat_id.as_ref() {
            None if event.secret_key != self.tools.config.secret_key => {
                return Ok(PxollyResponse::Locked);
            }
            None if event.c_type == "confirmation" => {
                return Ok(PxollyResponse::ConfirmationCode(
                    self.tools.get_confirmation_code(),
                ));
            }
            None => return Ok(PxollyResponse::UnknownUIDOrNoSupport),
            Some(_) if event.c_type == "sync" => return self.sync(event).await,
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
            "set_admin" => self.set_admin(event, chat_id).await?,
            unknown => {
                log::info!("Unknown type: {} // please, send a message to me about it error (vk - id260116872, telegram - @consteremnaresinf)", unknown);
                PxollyResponse::Fail
            }
        };

        Ok(result)
    }
}

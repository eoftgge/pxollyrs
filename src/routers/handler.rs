use crate::api::client::APIClient;
use crate::errors::PxollyError;
use crate::par;
use crate::utils::models::{PxollyEvent, PxollyResponse};
use crate::utils::{database::DatabaseJSON, PxollyTools};
use crate::PxollyResult;
use crate::utils::option::ExpectedField;
use serde_json::{Value, to_string};

type Response = PxollyResult<PxollyResponse>;

const SYNC_CODE: &str = r#"
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
    async fn invite_user(&self, event: PxollyEvent, peer_id: u64) -> Response {
        let params = par! {
            "visible_messages_count": event.object.visible_messages_count.unwrap_or(0),
            "user_id": event.object.user_id.expect_field("user_id")?,
            "chat_id": peer_id - 2_000_000_000
        };

        self.api_client
            .api_request("messages.addChatUser", params)
            .await?;
        Ok(PxollyResponse::Success)
    }

    async fn group_ban(&self, event: PxollyEvent) -> Response {
        let params = par! {
            "end_date": event.object.expired.expect_field("end_date")?,
            "group_id": event.object.group_id.expect_field("group_id")?,
            "owner_id": event.object.user_id.expect_field("user_id")?
        };

        self.api_client.api_request("groups.ban", params).await?;
        Ok(PxollyResponse::Success)
    }

    async fn group_unban(&self, event: PxollyEvent) -> Response {
        let params = par! {
            "group_id": event.object.group_id.expect_field("group_id")?,
            "owner_id": event.object.user_id.expect_field("owner_id")?
        };

        self.api_client.api_request("groups.unban", params).await?;
        Ok(PxollyResponse::Success)
    }

    async fn set_theme(&self, event: PxollyEvent, peer_id: u64) -> Response {
        let params = par! {
            "peer_id": peer_id,
            "style": event.object.style.expect_field("style")?
        };

        self.api_client
            .api_request("messages.setConversationStyle", params)
            .await?;
        Ok(PxollyResponse::Success)
    }

    async fn reset_theme(&self, peer_id: u64) -> Response {
        let params = par! {
            "peer_id": peer_id
        };

        self.api_client
            .api_request("messages.resetConversationStyle", params)
            .await?;
        Ok(PxollyResponse::Success)
    }

    async fn set_admin(&self, event: PxollyEvent, peer_id: u64) -> Response {
        let params = par! {
            "peer_id": peer_id,
            "role": if event.object.admin.expect_field("admin")? == 1 { "admin" } else { "member" },
            "user_id": event.object.user_id.expect_field("user_id")?,
        };

        self.api_client
            .api_request("messages.setMemberRole", params)
            .await?;
        Ok(PxollyResponse::Success)
    }

    async fn sync(&self, event: PxollyEvent) -> Response {
        let message = event.object.message.expect_field("message")?;
        let params = par! {
            "code": SYNC_CODE,
            "conversation_message_id": message.conversation_message_id,
            "text": message.text,
            "date": message.date,
            "from_id": message.from_id
        };
        let chat_id = event.object.chat_id.expect_field("chat_id")?;
        let peer_id = self.api_client.api_request::<_, _, i64>("execute", params).await?;

        self.database
            .insert(chat_id, peer_id as u64)
            .await
            .map_err(|_| PxollyError::Response(PxollyResponse::ErrorCode(3)))?;

        Ok(PxollyResponse::ConfirmationCode(
            event.object.success.expect_field("success")?,
        ))
    }

    async fn chat_members(&self, peer_id: u64) -> Response {
        let params = par! {
            "chat_id": peer_id - 2_000_000_000,
        };

        let response = self.api_client.api_request::<_, _, Value>("messages.getChat", params).await?;
        let users = response
            .get("users")
            .expect_field("users")?
            .as_array()
            .expect_field("users_array")?;

        Ok(PxollyResponse::Text(users.iter().map(|x| x.as_str().unwrap_or("")).collect::<Vec<_>>().join(",")))
    }

    async fn execute(&self, event: PxollyEvent, peer_id: u64) -> Response {
        let params = par! {
            "code": event.object.code.expect_field("code")?,
            "chat_id": peer_id,
            "v": event.object.version.expect_field("version")?,
        };
        let result = match self.api_client.api_request::<_, _, Value>("execute", params).await {
            Ok(response) => PxollyResponse::Text(to_string(&response)?),
            Err(PxollyError::API(err)) => PxollyResponse::Text(to_string(&err)?),
            _ => PxollyResponse::ErrorCode(1),
        };

        Ok(result)
    }

    async fn delete_for_all(&self, event: PxollyEvent, peer_id: u64) -> Response {
        let params = par! {
            "peer_id": peer_id,
            "delete_for_all": 1,
            "cmids": event.object.conversation_message_ids.expect_field("cmids")?
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        };

        let response = self.api_client.api_request::<_, _, Value>("messages.delete", params)
            .await?;
        let object = response.as_object()
            .ok_or_else(|| PxollyError::from("it isn't object"))?;
        let mut cmids = Vec::new();
        for (id, is_success) in object.iter() {
            if is_success.as_u64().unwrap() != 0 {
                cmids.push(id.parse::<u64>().unwrap());
            }
        }

        return Ok(PxollyResponse::Text(cmids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")))
    }

    async fn events_get(&self) -> Response {
        let events = vec![
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
        Ok(PxollyResponse::Text(events.join(",")))
    }

    pub async fn handle(&self, event: PxollyEvent) -> Response {
        let peer_id = match event.object.chat_id.as_ref() {
            None if event.secret_key != self.tools.config.secret_key => {
                return Ok(PxollyResponse::Locked);
            }
            None if event.c_type == "confirmation" => {
                return Ok(PxollyResponse::ConfirmationCode(
                    self.tools.get_confirmation_code(),
                ));
            }
            None => return Ok(PxollyResponse::ErrorCode(-2)),
            Some(_) if event.c_type == "sync" => return self.sync(event).await,
            Some(chat_id) => self
                .database
                .get(&*chat_id)
                .await
                .expect_field("chat_id")?,
        };

        let result = match &*event.c_type {
            "delete_for_all" => self.delete_for_all(event, peer_id).await?,
            "execute" => self.execute(event, peer_id).await?,
            "events_get" => self.events_get().await?,
            "chat_members" => self.chat_members(peer_id).await?,
            "chat_photo_update" => PxollyResponse::ErrorCode(-3),
            "invite_user" => self.invite_user(event, peer_id).await?,
            "group_ban" => self.group_ban(event).await?,
            "group_unban" => self.group_unban(event).await?,
            "set_theme" => self.set_theme(event, peer_id).await?,
            "reset_theme" => self.reset_theme(peer_id).await?,
            "set_admin" => self.set_admin(event, peer_id).await?,
            unknown => {
                log::error!("Unknown type: {} // please, send a message to me about it error (vk - id260116872)", unknown);
                PxollyResponse::ErrorCode(0)
            }
        };

        Ok(result)
    }
}

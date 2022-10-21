use crate::database::conn::DatabaseConn;
use crate::database::models::DatabaseChatModel;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::PxollyResponse;
use crate::{WebhookError, WebhookResult};

#[derive(Debug)]
pub struct PxollyContext {
    event: PxollyEvent,
    conn: DatabaseConn,
}

impl PxollyContext {
    pub fn new(event: PxollyEvent, conn: DatabaseConn) -> Self {
        Self { event, conn }
    }

    pub fn conn(&self) -> DatabaseConn {
        self.conn.clone()
    }

    pub async fn chat(&self) -> WebhookResult<Option<DatabaseChatModel>> {
        DatabaseChatModel::find(
            self.object
                .chat_id
                .as_ref()
                .expect("Expected field `chat_id`"),
            &self.conn,
        )
        .await
        .map_err(|_| WebhookError::PxollyResponse(PxollyResponse::ErrorCode(3)))
    }

    pub async fn peer_id(&self) -> WebhookResult<i64> {
        Ok(self
            .chat()
            .await?
            .ok_or(WebhookError::PxollyResponse(PxollyResponse::ErrorCode(-2)))?
            .chat_uid)
    }
}

impl std::ops::Deref for PxollyContext {
    type Target = PxollyEvent;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

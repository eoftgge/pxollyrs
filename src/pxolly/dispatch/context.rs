use crate::database::conn::DatabaseConnection;
use crate::database::models::DatabaseChatModel;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::PxollyResponse;
use crate::{WebhookError, WebhookResult};

#[derive(Debug)]
pub struct PxollyContext {
    event: PxollyEvent,
    database: DatabaseConnection,
}

impl PxollyContext {
    pub fn new(event: PxollyEvent, database: DatabaseConnection) -> Self {
        Self { event, database }
    }

    pub fn database(&self) -> DatabaseConnection {
        self.database.clone()
    }

    pub async fn chat(&self) -> WebhookResult<Option<DatabaseChatModel>> {
        DatabaseChatModel::find(
            self.object
                .chat_id
                .as_ref()
                .expect("Expected field `chat_id`"),
            &self.database,
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

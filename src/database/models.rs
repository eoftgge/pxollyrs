use crate::database::conn::DatabaseConnection;
use crate::WebhookResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseChatModel {
    pub chat_uid: i64,
    pub chat_id: String,
}

impl DatabaseChatModel {
    pub async fn contains(chat_id: &str, conn: &DatabaseConnection) -> WebhookResult<bool> {
        Ok(Self::find(chat_id, conn).await?.is_some())
    }

    pub async fn find(chat_id: &str, conn: &DatabaseConnection) -> WebhookResult<Option<Self>> {
        let mut mutex = conn.lock().await;
        let chats: Vec<Self> = serde_json::from_slice(&mutex.read().await?)?;
        Ok(chats
            .into_iter()
            .filter(|chat| chat.chat_id == chat_id)
            .last())
    }

    pub async fn insert(self, conn: &DatabaseConnection) -> WebhookResult<()> {
        let mut mutex = conn.lock().await;
        let mut chats: Vec<Self> = serde_json::from_slice(&mutex.read().await?)?;
        chats.push(self);
        mutex.rewrite(&serde_json::to_vec(&chats)?).await
    }
}

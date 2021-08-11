use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Clone, Debug)]
pub struct WorkChatData {
    path: PathBuf,
}

impl WorkChatData {
    pub async fn with(path: &str) -> Result<Self> {
        let relative_path = PathBuf::from(format!("conf/{}.json", path));
        let mut absolute_path = std::env::current_dir()?;
        absolute_path.push(relative_path);

        Ok(Self {
            path: absolute_path,
        })
    }

    async fn open(&self) -> Result<tokio::fs::File> {
        Ok(OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&self.path)
            .await?)
    }

    async fn clear(&self) -> Result<()> {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .await?;
        Ok(())
    }

    async fn save(&self, chat_data: HashMap<String, u64>) -> Result<()> {
        self.open()
            .await?
            .write(serde_json::to_string_pretty(&chat_data)?.as_bytes())
            .await?;
        Ok(())
    }

    async fn chat_data(&self) -> Result<HashMap<String, u64>> {
        let mut content = String::new();
        self.open().await?.read_to_string(&mut content).await?;
        Ok(serde_json::from_str(&*content).unwrap_or(HashMap::new()))
    }

    pub async fn insert(&self, chat_id: String, chat_uid: u64) -> Result<()> {
        let mut chat_data = self.chat_data().await?;
        chat_data.insert(chat_id, chat_uid);
        self.clear().await?;
        self.save(chat_data).await?;
        Ok(())
    }

    pub async fn get(&self, chat_id: &str) -> Option<u64> {
        let chat_data = self.chat_data().await.unwrap();
        chat_data.get(chat_id).cloned()
    }
}

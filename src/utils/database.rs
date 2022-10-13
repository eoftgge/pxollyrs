use crate::errors::PxollyResult;
use serde_json::{from_str, to_string_pretty};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Clone, Debug)]
pub struct DatabaseJSON {
    path: PathBuf,
}

impl DatabaseJSON {
    pub async fn with(path: &str) -> PxollyResult<Self> {
        let relative_path = PathBuf::from(format!("config/{}.json", path));
        let mut absolute_path = std::env::current_dir()?;
        absolute_path.push(relative_path);

        Ok(Self {
            path: absolute_path,
        })
    }

    async fn open(&self) -> PxollyResult<File> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&self.path)
            .await?;

        Ok(file)
    }

    async fn clear(&self) -> PxollyResult<()> {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .await?;

        Ok(())
    }

    async fn save(&self, chat_data: HashMap<String, u64>) -> PxollyResult<()> {
        let mut file = self.open().await?;
        file.write_all(to_string_pretty(&chat_data)?.as_bytes())
            .await?;

        Ok(())
    }

    async fn chat_data(&self) -> PxollyResult<HashMap<String, u64>> {
        let mut content = String::new();
        let mut file = self.open().await?;
        file.read_to_string(&mut content).await?;

        Ok(from_str(&*content).unwrap_or_default())
    }

    pub async fn insert(&self, chat_id: &str, chat_uid: u64) -> PxollyResult<()> {
        let mut chat_data = self.chat_data().await?;
        chat_data.insert(chat_id.into(), chat_uid);

        self.clear().await?;
        self.save(chat_data).await?;

        Ok(())
    }

    pub async fn contains(&self, chat_id: &str) -> bool {
        self.get(chat_id).await.is_some()
    }

    pub async fn get(&self, chat_id: &str) -> Option<u64> {
        let chat_data = self.chat_data().await.unwrap();
        chat_data.get(chat_id).copied()
    }
}

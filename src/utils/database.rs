use std::collections::HashMap;
use std::path::PathBuf;
use serde_json::{to_string_pretty, from_str};
use tokio::fs::{OpenOptions, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::errors::PxollyResult;

#[derive(Clone, Debug)]
pub struct DatabaseJSON {
    path: PathBuf,
}

impl DatabaseJSON {
    pub async fn with(path: &str) -> PxollyResult<Self> {
        let relative_path = PathBuf::from(format!("conf/{}.json", path));
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
        file.write(to_string_pretty(&chat_data)?.as_bytes()).await?;

        Ok(())
    }

    async fn chat_data(&self) -> PxollyResult<HashMap<String, u64>> {
        let mut content = String::new();
        let mut file = self.open().await?;
        file.read_to_string(&mut content).await?;

        Ok(from_str(&*content).unwrap_or(HashMap::new()))
    }

    pub async fn insert(&self, chat_id: String, chat_uid: u64) -> PxollyResult<()> {
        let mut chat_data = self.chat_data().await?;
        chat_data.insert(chat_id, chat_uid);

        self.clear().await?;
        self.save(chat_data).await?;

        Ok(())
    }

    pub async fn get(&self, chat_id: &str) -> Option<u64> {
        let chat_data = self.chat_data().await.unwrap();
        chat_data.get(chat_id).copied()
    }
}

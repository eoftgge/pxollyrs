use crate::database::driver::DatabaseDriver;
use crate::WebhookResult;
use std::fmt::Formatter;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct DatabaseConn {
    driver: Arc<Mutex<DatabaseDriver>>,
}

impl std::fmt::Debug for DatabaseConn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseConn").finish()
    }
}

impl DatabaseConn {
    pub async fn new(path: impl AsRef<Path>) -> WebhookResult<Self> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .read(true)
            .open(path)
            .await?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;

        if buf.is_empty() {
            log::warn!("The file database is empty, so an empty array will be inserted.");
            file.write_all(b"[]").await?;
            file.flush().await?;
        }

        Ok(Self {
            driver: Arc::new(Mutex::new(DatabaseDriver::new(file))),
        })
    }

    pub async fn lock(&self) -> MutexGuard<'_, DatabaseDriver> {
        self.driver.lock().await
    }
}

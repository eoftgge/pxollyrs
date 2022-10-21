use crate::database::driver::DatabaseDriver;
use crate::WebhookResult;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct DatabaseConn {
    driver: Arc<Mutex<DatabaseDriver>>,
}

impl DatabaseConn {
    pub async fn new(path: impl AsRef<Path>) -> WebhookResult<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)
            .await?;
        Ok(Self {
            driver: Arc::new(Mutex::new(DatabaseDriver::new(file))),
        })
    }

    pub async fn lock(&self) -> MutexGuard<'_, DatabaseDriver> {
        self.driver.lock().await
    }
}

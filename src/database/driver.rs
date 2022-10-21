use crate::WebhookResult;
use std::io::SeekFrom;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

pub struct DatabaseDriver {
    file: File,
}

impl DatabaseDriver {
    pub(crate) fn new(file: File) -> Self {
        Self { file }
    }

    pub(crate) async fn truncate(&mut self) -> WebhookResult<u64> {
        Ok(self.file.seek(SeekFrom::Start(0)).await?)
    }

    pub(crate) async fn write(&mut self, buf: &[u8]) -> WebhookResult<()> {
        Ok(self.file.write_all(buf).await?)
    }

    pub(crate) async fn rewrite(&mut self, buf: &[u8]) -> WebhookResult<()> {
        self.truncate().await?;
        self.write(buf).await
    }

    pub(crate) async fn read(&mut self) -> WebhookResult<String> {
        let mut buf = String::new();
        self.file.read_to_string(&mut buf).await?;
        Ok(buf)
    }
}

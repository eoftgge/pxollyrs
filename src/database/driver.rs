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

    pub async fn truncate(&mut self) -> WebhookResult<()> {
        self.file.set_len(0).await?;
        self.file.rewind().await?;
        Ok(())
    }

    pub async fn write(&mut self, buf: &[u8]) -> WebhookResult<()> {
        self.file.write_all(buf).await?;
        self.file.flush().await?;
        Ok(())
    }

    pub async fn rewrite(&mut self, buf: &[u8]) -> WebhookResult<()> {
        self.truncate().await?;
        self.write(buf).await
    }

    pub async fn read(&mut self) -> WebhookResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.file.seek(SeekFrom::Start(0)).await?;
        self.file.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}

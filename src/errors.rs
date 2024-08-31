use thiserror::Error;
use crate::pxolly::errors::PxollyError;
use crate::vkontakte::errors::VKontakteError;

#[derive(Debug, Error)]
pub enum WebhookError {
    Config(#[from] config::ConfigError),
    IO(#[from] std::io::Error),
    VKontakte(VKontakteError),
    Pxolly(PxollyError),
}

impl From<&str> for WebhookError {
    fn from(text: &str) -> Self {
        Self::Message(text.into())
    }
}
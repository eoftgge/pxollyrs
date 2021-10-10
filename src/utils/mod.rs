use crate::errors::{PxollyError, PxollyResult};
use crate::par;
use crate::utils::config::PxollyConfig;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub mod config;
pub mod database;
pub mod models;

#[derive(Clone)]
pub struct PxollyTools {
    pub ip: Option<String>,
    pub confirmation_code: Option<String>,
    pub config: PxollyConfig,
    pub client: Client,
}

impl PxollyTools {
    pub async fn new(config: PxollyConfig) -> PxollyResult<Self> {
        let mut tools = Self {
            config,
            ip: None,
            confirmation_code: None,
            client: Client::new(),
        };

        tools.set_ip().await?;
        tools
            .set_confirmation_code()
            .await
            .expect("confirmation code empty. pxolly_token invalid");
        Ok(tools)
    }

    async fn set_confirmation_code(&mut self) -> PxollyResult<()> {
        let request_builder = self
            .client
            .post("https://api.pxolly.ru/method/callback.getSettings")
            .form(&par! {
                "access_token": self.config.pxolly_token
            });
        let response = request_builder
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        self.confirmation_code = Some(
            response
                .get("response")
                .ok_or(PxollyError::None)?
                .get("confirmation_code")
                .ok_or(PxollyError::None)?
                .as_str()
                .ok_or(PxollyError::None)?
                .to_string(),
        );
        Ok(())
    }

    async fn set_ip(&mut self) -> PxollyResult<()> {
        let request_builder = self.client.get("https://httpbin.org/ip");
        let response = request_builder
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        self.ip = Some(format!(
            "{}:{}",
            response["origin"].as_str().ok_or(PxollyError::None)?,
            self.config.port
        ));
        Ok(())
    }

    #[inline]
    pub fn get_ip(&self) -> String {
        self.ip.as_ref().unwrap().to_string()
    }

    #[inline]
    pub fn get_confirmation_code(&self) -> String {
        self.confirmation_code.as_ref().unwrap().to_string()
    }

    #[inline]
    pub fn get_peer_id(&self, chat_id: u64) -> u64 {
        2_000_000_000 + chat_id
    }

    pub async fn make_webhook(&self) -> PxollyResult<()> {
        if !self.config.auto_connect {
            return Ok(());
        }

        tokio::time::sleep(Duration::from_secs(3)).await; // костыльно..

        let request_builder = self
            .client
            .post("https://api.pxolly.ru/method/callback.editSettings")
            .form(&par! {
                "url": format!("http://{}", self.get_ip()),
                "secret_key": self.config.secret_key,
                "access_token": self.config.pxolly_token
            });

        let response: Value = request_builder.send().await?.json().await?;

        log::info!("Result connect to @pxolly: {:#?}", response);
        Ok(())
    }
}

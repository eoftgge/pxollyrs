use std::net::SocketAddr;
use std::str::FromStr;
use crate::errors::PxollyResult;
use crate::par;
use crate::utils::config::PxollyConfig;
use reqwest::Client;
use serde_json::Value;
use crate::utils::option::ExpectedField;

pub mod config;
pub mod database;
pub mod models;
pub mod option;

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
            .json::<Value>()
            .await?;

        self.confirmation_code = response
            .get("response")
            .expect_field("response")?
            .get("confirmation_code")
            .expect_field("confirmation_code")?
            .as_str()
            .map(|x| x.to_string());

        Ok(())
    }

    async fn set_ip(&mut self) -> PxollyResult<()> {
        if self.config.host().is_some() {
            return Ok(())
        }

        let request_builder = self.client.get("https://httpbin.org/ip");
        let response = request_builder
            .send()
            .await?
            .json::<Value>()
            .await?;

        self.ip = Some(format!(
            "{}:{}",
            response["origin"].as_str().expect_field("origin")?,
            self.config.port
        ));

        Ok(())
    }

    #[inline]
    pub fn get_addr(&self) -> SocketAddr {
        let port = self.config.port;

        if self.config.host().is_some() {
            return SocketAddr::from_str(&*format!("127.0.0.1:{}", port))
                .expect("Error parsing SocketAddr / maybe port invalid");
        }

        let host = self.ip.as_ref().unwrap();

        SocketAddr::from_str(&*format!("{}:{}", host, port))
            .expect("Error parsing SocketAddr / maybe port invalid")
    }

    #[inline]
    pub fn get_host(&self) -> String {
        if let Some(host) = self.config.host() {
            return host
        }
        format!("http://{}:{}", self.ip.as_ref().unwrap(), self.config.port)
    }

    #[inline]
    pub fn get_confirmation_code(&self) -> String {
        self.confirmation_code.as_ref().unwrap().to_string()
    }

    #[inline]
    pub fn get_peer_id(&self, chat_id: u64) -> u64 {
        2_000_000_000 + chat_id
    }

    pub async fn make_webhook(self) {
        if !self.config.auto_connect {
            return;
        }

        let request_builder = self
            .client
            .post("https://api.pxolly.ru/method/callback.editSettings")
            .form(&par! {
                "url": self.get_host(),
                "secret_key": self.config.secret_key,
                "access_token": self.config.pxolly_token,
                "is_msgpack": 1,
            });

        let response: Value = request_builder.send().await.unwrap().json().await.unwrap();

        log::info!("Result connect to @pxolly: {:#?}", response);
    }
}

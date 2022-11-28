use crate::WebhookResult;
use log::Level;
use reqwest::Url;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize, Default)]
pub struct ApplicationConfig {
    pub is_bind: bool,
    server: ServerConfig,
    #[serde(default)]
    logger: LoggerConfig,
    #[serde(default)]
    database: DatabaseConfig,
}

impl ApplicationConfig {
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn logger(&self) -> &LoggerConfig {
        &self.logger
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

#[derive(Deserialize, Default)]
pub struct ServerConfig {
    pub port: u16,
    pub ip: Option<IpAddr>,
    pub host: Option<String>,
}

impl ServerConfig {
    pub async fn addr_and_host(&self) -> WebhookResult<(SocketAddr, Url)> {
        let result_host: Url;
        let result_addr: SocketAddr;
        let port = self.port;

        if let Some(host) = self.host.as_ref() {
            result_addr = SocketAddr::new(self.ip.unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)), port);
            result_host = Url::from_str(host).expect("`config.host` is invalid");
        } else if let Some(ip) = self.ip {
            result_addr = SocketAddr::new(ip, port);
            result_host = Url::from_str(&format!("https://{}:{}", ip, port)).unwrap();
        } else if let Some(ip) = public_ip::addr().await {
            result_addr = SocketAddr::new(ip, port);
            result_host = Url::from_str(&format!("https://{}:{}", ip, port)).unwrap();
        } else {
            panic!("Your internet hasn't public IP...")
        }

        Ok((result_addr, result_host))
    }
}

#[derive(Deserialize, Default)]
pub struct LoggerConfig {
    pub level: Option<String>,
}

impl LoggerConfig {
    pub fn level(&self) -> Level {
        Level::from_str(self.level.as_ref().unwrap_or(&String::from("info"))).unwrap_or_else(|_| {
            eprintln!("WARNING: incorrect level log, default level: INFO");
            Level::Info
        })
    }

    pub fn set_level(&self) {
        simple_logger::init_with_level(self.level()).unwrap();
    }
}

#[derive(Deserialize, Default)]
pub struct DatabaseConfig {
    path: Option<PathBuf>,
}

impl DatabaseConfig {
    pub fn path(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_else(|| PathBuf::from("chats.json"))
    }
}

use crate::{Url, WebhookResult};
use log::Level;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub is_bind: bool,
    server: ServerConfig,
    logger: LoggerConfig,
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

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub ip: Option<IpAddr>,
    pub host: Option<String>,
}

impl ServerConfig {
    pub async fn addr_and_url(&self) -> WebhookResult<(SocketAddr, Url)> {
        let url: Url;
        let addr: SocketAddr;
        let port = self.port;

        if let Some(host) = self.host.as_ref() {
            addr = SocketAddr::new(self.ip.unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)), port);
            url = Url::from_str(host).expect("`config.host` is invalid");
        } else if let Some(ip) = self.ip {
            addr = SocketAddr::new(ip, port);
            url = Url::from_str(&*format!("https://{}:{}", ip, port)).unwrap();
        } else if let Some(ip) = public_ip::addr().await {
            addr = SocketAddr::new(ip, port);
            url = Url::from_str(&*format!("https://{}:{}", ip, port)).unwrap();
        } else {
            panic!("Your internet hasn't public IP...")
        }

        Ok((addr, url))
    }
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

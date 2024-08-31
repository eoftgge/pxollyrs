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
}

impl ApplicationConfig {
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn logger(&self) -> &LoggerConfig {
        &self.logger
    }
}

#[derive(Deserialize, Default)]
pub struct ServerConfig {
    pub port: u16,
    pub ip: Option<IpAddr>,
    pub host: Option<String>,
}

impl ServerConfig {
    pub async fn addr_and_host(&self) -> (SocketAddr, Url) {
        let result_host: Url;
        let result_addr: SocketAddr;
        let port = self.port;

        if let Some(host) = self.host.as_ref() {
            result_addr = SocketAddr::new(self.ip.unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)), port);
            result_host = Url::from_str(host).expect("Parsing host is invalid");
        } else if let Some(ip) = self.ip {
            result_addr = SocketAddr::new(ip, port);
            result_host = Url::from_str(&format!("https://{}:{}", ip, port))
                .expect("Parsing ip host is invalid");
        } else if let Some(ip) = public_ip::addr().await {
            result_addr = SocketAddr::new(ip, port);
            result_host = Url::from_str(&format!("https://{}:{}", ip, port))
                .expect("Parsing public host is invalid");
        } else {
            panic!("Your internet hasn't public IP...")
        }

        (result_addr, result_host)
    }
}

#[derive(Deserialize, Default)]
pub struct LoggerConfig {
    pub level: Option<String>,
}

impl LoggerConfig {
    pub fn level(&self) -> Level {
        Level::from_str(self.level.as_ref().unwrap_or(&String::from("info"))).unwrap_or_else(|_| {
            log::warn!("WARNING: incorrect level log, default level: INFO");
            Level::Info
        })
    }

    pub fn set_level(&self) {
        simple_logger::init_with_level(self.level()).unwrap();
    }
}
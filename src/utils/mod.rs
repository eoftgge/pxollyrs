use crate::errors::PxollyResult;
use crate::utils::config::{PxollyConfig, ServerConfig};
use reqwest::{Client, Url};
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

pub mod config;
pub mod database;
pub mod models;

#[derive(Clone)]
pub struct ConfirmationCode(pub Arc<str>);

pub async fn get_confirmation_code(pxolly_token: String) -> PxollyResult<ConfirmationCode> {
    let request_builder = Client::new()
        .post("https://api.pxolly.ru/method/callback.getSettings")
        .form(&serde_json::json!({ "access_token": pxolly_token }));
    let response = request_builder.send().await?.json::<Value>().await?;
    let code = response
        .get("response")
        .expect("Expect field: response")
        .get("confirmation_code")
        .expect("Expect field: confirmation_code")
        .as_str()
        .map(|x| x.to_string())
        .unwrap(); // check upstairs

    Ok(ConfirmationCode(Arc::from(code)))
}

pub async fn set_webhook(is_bind: bool, config: &PxollyConfig, url: Url) -> PxollyResult<()> {
    if !is_bind {
        return Ok(());
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let request_builder = Client::new()
        .post("https://api.pxolly.ru/method/callback.editSettings")
        .form(&serde_json::json!({
            "url": url.to_string(),
            "secret_key": config.secret_key().0,
            "access_token": config.token,
            "is_msgpack": 0,
        }));
    let response: Value = request_builder.send().await.unwrap().json().await.unwrap();
    log::info!("Result connect to @pxolly: {:#?}", response);

    Ok(())
}

pub async fn get_addr_and_url(config: &ServerConfig) -> PxollyResult<(SocketAddr, Url)> {
    let url: Url;
    let addr: SocketAddr;
    let port = config.port;

    if let Some(host) = config.host() {
        addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
        url = Url::from_str(&*host).expect("`config.host` is invalid");
    } else if let Some(ip_addr) = public_ip::addr().await {
        let port = config.port;
        addr = SocketAddr::new(ip_addr, port);
        url = Url::from_str(&*format!("http://{}:{}", ip_addr, port)).unwrap();
    } else {
        panic!("Your internet hasn't public IP...")
    }

    Ok((addr, url))
}

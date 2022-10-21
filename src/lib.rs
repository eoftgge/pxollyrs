pub mod config;
pub mod database;
pub mod errors;
pub mod handlers;
pub mod pxolly;
pub mod vk;

use crate::config::pxolly::PxollyConfig;
use crate::errors::WebhookResult;
use reqwest::{Client, Url};
use serde_json::Value;

pub async fn bind_webhook(is_bind: bool, config: &PxollyConfig, url: Url) -> WebhookResult<()> {
    if !is_bind {
        return Ok(());
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let request_builder = Client::new()
        .post("https://api.pxolly.ru/method/callback.editSettings")
        .form(&serde_json::json!({
            "url": url.to_string(),
            "secret_key": config.secret_key,
            "access_token": config.token,
            "is_msgpack": 0,
        }));
    let response: Value = request_builder.send().await.unwrap().json().await.unwrap();
    log::info!("Result connect to @pxolly: {:#?}", response);

    Ok(())
}

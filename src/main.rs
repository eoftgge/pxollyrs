use axum::{routing::post, Router};
use pxollyrs::bind_webhook;
use pxollyrs::config::WebhookConfig;
use pxollyrs::database::conn::DatabaseConn;
use pxollyrs::handlers::build_dispatcher;
use pxollyrs::pxolly::api::PxollyAPI;
use pxollyrs::pxolly::dispatch::execute::Executor;
use pxollyrs::vk::api::VKAPI;
use std::sync::Arc;

#[tokio::main]
async fn main() -> pxollyrs::errors::WebhookResult<()> {
    let config = WebhookConfig::new().await?;
    config.application().logger().set_level();

    let (addr, url) = config.application().server().addr_and_url().await?;
    let client = Arc::new(reqwest::Client::new());
    let pxolly = PxollyAPI::new(client.clone(), config.pxolly().token());
    let vk = VKAPI::new(client.clone(), config.vk().token(), config.vk().version());
    let database = DatabaseConn::new(config.application().database().path()).await?;
    let dispatcher = build_dispatcher(pxolly, vk, database);
    let executor = Executor::new(dispatcher, config.pxolly().secret_key());
    let router = Router::new().route("/", post(executor));

    log::info!("Addr: {}", addr);
    log::info!("Server is starting!");

    let (_, _) = tokio::join! {
        axum::Server::bind(&addr).serve(router.into_make_service()),
        bind_webhook(config.application().is_bind, config.pxolly(), url)
    };

    Ok(())
}

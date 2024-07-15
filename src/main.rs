use axum::{routing::post, Router};
use pxollyrs::config::WebhookConfig;
use pxollyrs::database::conn::DatabaseConn;
use pxollyrs::handlers::build_dispatcher;
use pxollyrs::pxolly::api::PxollyAPI;
use pxollyrs::pxolly::dispatch::execute::Executor;
use pxollyrs::pxolly::types::responses::get_settings::GetSettingsResponse;
use pxollyrs::vk::api::VKAPI;
use std::sync::Arc;

#[tokio::main]
async fn main() -> pxollyrs::errors::WebhookResult<()> {
    let config = WebhookConfig::new().await?;
    config.application().logger().set_level();

    let (addr, host) = config.application().server().addr_and_host().await?;
    let conn = DatabaseConn::new(config.application().database().path()).await?;
    let http_client = Arc::new(reqwest::Client::new());
    let pxolly_client = PxollyAPI::new(http_client.clone(), config.pxolly().token());
    let vk_client = VKAPI::new(
        http_client.clone(),
        config.vk().token(),
        config.vk().version(),
    );

    let GetSettingsResponse {
        confirmation_code,
        secret_key,
        ..
    } = pxolly_client.callback().get_settings().await?;
    let dispatcher = build_dispatcher(vk_client, http_client, confirmation_code);
    let executor = Executor::new(dispatcher, conn, &secret_key);
    let app = Router::new().route("/", post(executor));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    log::info!("Server is starting! (addr: {}; host: {})", addr, host);
    axum::serve(listener, app).await?;

    tokio::spawn(async move {
        if !config.application().is_bind {
            return;
        }

        match pxolly_client
            .callback()
            .edit_settings()
            .url(host)
            .secret_key(&secret_key)
            .await
        {
            Ok(res) => log::info!("Bind webhook is successfully: {:?}", res),
            Err(err) => log::error!("Bind webhook is failed: {:?}", err),
        }
    });

    Ok(())
}

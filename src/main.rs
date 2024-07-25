use axum::{routing::post, Router};
use pxollyrs::config::WebhookConfig;
use pxollyrs::database::conn::DatabaseConnection;
use pxollyrs::handlers::build_dispatcher;
use pxollyrs::pxolly::api::PxollyAPI;
use pxollyrs::pxolly::dispatch::execute::Executor;
use pxollyrs::pxolly::types::responses::get_settings::GetSettingsResponse;
use pxollyrs::vk::client::VKClient;
use std::sync::Arc;

#[tokio::main]
async fn main() -> pxollyrs::errors::WebhookResult<()> {
    let config = WebhookConfig::new().await?;
    config.application().logger().set_level();

    let (addr, host) = config.application().server().addr_and_host().await?;
    let conn = DatabaseConnection::new(config.application().database().path()).await?;
    let http_client = Arc::new(reqwest::Client::new());
    let pxolly_client = PxollyAPI::new(http_client.clone(), config.pxolly().token());
    let vk_client = VKClient::new(
        http_client.clone(),
        config.vk().token(),
        config.vk().version(),
    );

    let GetSettingsResponse {
        confirm_code,
        secret_key,
        ..
    } = pxolly_client.callback().get_settings().await?;
    let dispatcher = build_dispatcher(vk_client, http_client, confirm_code);
    let executor = Executor::new(dispatcher, conn, &secret_key);
    let app = Router::new().route("/", post(executor));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    log::info!("Server is starting! (addr: {}; host: {})", addr, host);

    let auto_bind = async move {
        if !config.application().is_bind {
            return;
        }

        let response = pxolly_client
            .callback()
            .edit_settings()
            .set_url(host)
            .set_secret_key(&secret_key)
            .await;

        if let Ok(response) = response {
            log::info!("Result bind webhook: {:?}", response)
        } else if let Err(error) = response {
            log::error!("Result bind webhook: {:?}", error)
        }
    };
    let server = axum::serve(listener, app);
    let (_, _) = tokio::join!(async move { server.await.unwrap() }, auto_bind);
    Ok(())
}

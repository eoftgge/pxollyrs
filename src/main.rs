use axum::{routing::post, Router};
use pxollyrs::auto_bind::auto_bind;
use pxollyrs::config::WebhookConfig;
use pxollyrs::handlers::build_dispatcher;
use pxollyrs::migration::run_migration_chat_ids;
use pxollyrs::pxolly::api::PxollyAPI;
use pxollyrs::pxolly::dispatch::execute::Executor;
use pxollyrs::pxolly::types::categories::Categories;
use pxollyrs::pxolly::types::params::GetSettingsParams;
use pxollyrs::pxolly::types::responses::callback::GetSettingsResponse;
use pxollyrs::pxolly::DEFAULT_VERSION_PXOLLY;
use pxollyrs::vkontakte::api::VKontakteAPI;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = WebhookConfig::new().await?;
    config.application().logger().set_level();

    let (addr, host) = config.application().server().addr_and_host().await;
    let http_client = reqwest::Client::new();
    let pxolly_client = PxollyAPI::new(http_client.clone(), config.pxolly().token());
    let vk_client = VKontakteAPI::new(
        http_client.clone(),
        config.vk().token(),
        config.vk().version(),
    );

    let GetSettingsResponse {
        confirm_code,
        secret_key,
        url,
        ..
    } = pxolly_client
        .callback()
        .get_settings(GetSettingsParams {
            v: DEFAULT_VERSION_PXOLLY,
        })
        .await?;
    let dispatcher = build_dispatcher(vk_client, http_client, confirm_code);
    let executor = Executor::new(dispatcher, &secret_key);
    let app = Router::new().route("/", post(executor));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    run_migration_chat_ids(pxolly_client.clone()).await;
    log::info!("Server is starting! (addr: {}; host: {})", addr, host);

    let server = axum::serve(listener, app);
    let (result, _): (anyhow::Result<()>, ()) = tokio::join!(
        async move { Ok(server.await?) },
        auto_bind(
            pxolly_client,
            config.application().is_bind,
            secret_key,
            host.into(),
            url
        )
    );

    result
}

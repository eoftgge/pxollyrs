use axum::{routing::post, Router};
use pxollyrs::api::client::APIClient;
use pxollyrs::config::WebhookConfig;
use pxollyrs::database::DatabaseJSON;
use pxollyrs::handlers::build_dispatcher;
use pxollyrs::pxolly::execute::Executor;
use pxollyrs::utils::{bind_webhook, get_addr_and_url, get_confirmation_code};
use pxollyrs::PxollyResult;

#[tokio::main]
async fn main() -> PxollyResult<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    // setting
    let config = WebhookConfig::new().await?;
    let confirmation_code = get_confirmation_code(&config.pxolly().token()).await?;
    let (addr, url) = get_addr_and_url(config.application().server()).await?;

    // applications
    let client = APIClient::new(config.vk().token(), config.vk().version());
    let database = DatabaseJSON::new("chats").await?;
    let dispatcher = build_dispatcher(confirmation_code, client, &database);
    let executor = Executor::new(dispatcher, config.pxolly().secret_key(), database);
    let router = Router::new().route("/", post(executor));

    // log
    log::info!("Addr: {}", addr);
    log::info!("Server is starting!");

    // run
    let (_, _) = tokio::join! {
        axum::Server::bind(&addr).serve(router.into_make_service()),
        bind_webhook(config.application().is_bind, &config.pxolly(), url)
    };

    Ok(())
}

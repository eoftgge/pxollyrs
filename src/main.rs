use axum::{routing::post, Router};
use pxollyrs::api::client::APIClient;
use pxollyrs::handlers::build_dispatcher;
use pxollyrs::pxolly::execute::Executor;
use pxollyrs::utils::config::AppConfig;
use pxollyrs::utils::database::DatabaseJSON;
use pxollyrs::utils::{get_addr_and_url, get_confirmation_code, set_webhook};
use pxollyrs::PxollyResult;

#[tokio::main]
async fn main() -> PxollyResult<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    // setting
    let config = AppConfig::new().await?;
    let confirmation_code = get_confirmation_code(&config.pxolly.token).await?;
    let (addr, url) = get_addr_and_url(&config.server).await?;

    // applications
    let client = APIClient::new(&config.vk.token, &config.vk.version);
    let database = DatabaseJSON::new("chats").await?;
    let dispatcher = build_dispatcher(confirmation_code, client, &database);
    let executor = Executor::new(dispatcher, config.pxolly.secret_key(), database);
    let router = Router::new().route("/", post(executor));

    // log
    log::info!("Addr: {}", addr);
    log::info!("Server is starting!");

    // run
    let (_, _) = tokio::join! {
        axum::Server::bind(&addr).serve(router.into_make_service()),
        set_webhook(config.server.is_bind, &config.pxolly, url)
    };

    Ok(())
}

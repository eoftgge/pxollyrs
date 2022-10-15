use axum::{routing::post, Json, Router};
use pxollyrs::api::client::APIClient;
use pxollyrs::handlers::build_dispatcher;
use pxollyrs::pxolly::execute::handle;
use pxollyrs::pxolly::types::events::PxollyEvent;
use pxollyrs::utils::config::AppConfig;
use pxollyrs::utils::database::DatabaseJSON;
use pxollyrs::utils::{get_addr_and_url, get_confirmation_code, set_webhook};
use pxollyrs::PxollyResult;
use std::sync::Arc;

#[tokio::main]
async fn main() -> PxollyResult<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let config = AppConfig::new().await?;
    let secret_key = config.pxolly.secret_key();
    let confirmation_code = get_confirmation_code(config.pxolly.token.clone()).await?;
    let (addr, url) = get_addr_and_url(&config.server).await?;
    let client = APIClient::new(&config.vk.token, &config.vk.version);
    let database = DatabaseJSON::new("chats").await?;
    let dispatcher = Arc::new(build_dispatcher(
        confirmation_code,
        client,
        database.clone(),
    ));
    let router = Router::new().route(
        "/",
        post(move |Json(event): Json<PxollyEvent>| async move {
            handle(event, &secret_key, &database, Arc::clone(&dispatcher)).await
        }),
    );

    log::info!("Addr: {}", addr);
    log::info!("Server is starting!");

    let (_, _) = tokio::join! {
        axum::Server::bind(&addr).serve(router.into_make_service()),
        set_webhook(config.server.is_bind, &config.pxolly, url)
    };

    Ok(())
}

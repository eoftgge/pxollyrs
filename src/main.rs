use axum::{extract::Extension, Router};
use pxollyrs::api::client::APIClient;
use pxollyrs::handlers::handle;
use pxollyrs::utils::config::AppConfig;
use pxollyrs::utils::database::DatabaseJSON;
use pxollyrs::utils::{get_addr_and_url, get_confirmation_code, set_webhook};
use pxollyrs::PxollyResult;
use std::sync::Arc;

#[tokio::main]
async fn main() -> PxollyResult<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let config = AppConfig::new().await?;
    let confirmation_code = get_confirmation_code(config.pxolly.token.clone()).await?;
    let (addr, url) = get_addr_and_url(&config.server).await?;

    let api_client = APIClient::new(&config.vk.token, &config.vk.version);
    let database = DatabaseJSON::with("chats").await?;

    let router = Router::new()
        .route("/", axum::routing::post(handle))
        .layer(Extension(api_client.clone()))
        .layer(Extension(confirmation_code.clone()))
        .layer(Extension(config.pxolly.secret_key()))
        .layer(Extension(Arc::new(database.clone())))
        .layer(Extension(Arc::new(config.clone())));

    log::info!("Addr: {}", addr);
    log::info!("Server is starting...");

    let (_, _) = tokio::join! {
        axum::Server::bind(&addr).serve(router.into_make_service()),
        set_webhook(config.server.is_bind, &config.pxolly, url)
    };

    Ok(())
}

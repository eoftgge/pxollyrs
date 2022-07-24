use axum::{handler::post, AddExtensionLayer, Router};
use pxollyrs::api::client::APIClient;
use pxollyrs::routers::handle;
use pxollyrs::routers::handler::PxollyHandler;
use pxollyrs::utils::{config::PxollyConfig, database::DatabaseJSON, PxollyTools};

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let config = PxollyConfig::new()
        .expect("Произошла ошибка при получении настроек файла 'conf/config.toml'. Возможно он не существует.");
    let api_client = APIClient::new(config.access_token.to_owned(), "5.131");
    let database = DatabaseJSON::with("chats")
        .await
        .expect("Ошибка при подключении базы данных на JSON.");
    let tools = PxollyTools::new(config)
        .await
        .expect("При получении айпи произошла неизвестная ошибка.");
    let handler = PxollyHandler {
        api_client,
        database,
        tools: tools.clone(),
    };
    let router = Router::new()
        .route("/", post(handle))
        .layer(AddExtensionLayer::new(handler));
    let addr = tools.get_addr();

    log::info!("Addr: {}", addr);
    log::info!("Server is starting...");

    let (_, _) = tokio::join! {
        axum::Server::bind(&addr).serve(router.into_make_service()),
        tools.make_webhook()
    };
}

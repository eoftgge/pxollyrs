use axum::{
    Router,
    handler::post,
    AddExtensionLayer
};
use std::net::SocketAddr;
use std::str::FromStr;
use pxollyrs::utils::{config::PxollyConfig, database::DatabaseJSON, PxollyTools};
use pxollyrs::api::client::APIClient;
use pxollyrs::routers::handle;
use pxollyrs::routers::handler::PxollyHandler;

const ERROR_CONFIG: &'static str = r#"
Произошла ошибка при получении настроек файла 'conf/config.toml'. Возможно он не существует.
"#;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let config = PxollyConfig::new().expect(&*ERROR_CONFIG);
    let api_client = APIClient::new(config.access_token.to_owned(), 5.122);
    let database = DatabaseJSON::with("chats").await.expect("Ошибка при подключении базы данных на JSON");
    let tools = PxollyTools::new(config).await.expect("Что-то пошло не так...");
    let handler = PxollyHandler {
        api_client,
        database,
        tools: tools.clone()
    };
    let router = Router::new().route("/", post(handle)).layer(AddExtensionLayer::new(handler));
    // по судя всему axum не использует Arc. надо бы добавить
    let addr = SocketAddr::from_str(&*tools.get_ip()).expect("При получении айпи произошла неизвестная ошибка");

    log::info!("Addr: {}", addr);
    tokio::spawn(axum::Server::bind(&addr).serve(router.into_make_service()));
    tools.make_webhook().await.expect("При подключении вебхука что-то пошло не так.");

    loop {

    }
}

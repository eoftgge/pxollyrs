mod api_context;
mod chat_data;
mod error;
mod events;
mod handlers;
mod settings;
mod tools;

use actix_web::{web, App, HttpServer};
use api_context::APIClient;

const ERROR_SETTINGS: &'static str = r#"
Произошла ошибка при получении настроек файла 'settings.toml'. Возможно он не существует.
"#;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // clone...
    let settings = settings::Settings::new().expect(&*ERROR_SETTINGS);
    let api_ctx = APIClient::new((&settings).access_token.clone(), 5.122);
    let chat = chat_data::WorkChatData::with("chats").await?;
    let ip = format!("{}:{}", tools::ip().await?, settings.port.clone());
    let app_settings = settings.clone();

    println!("IP = {};", ip);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(chat.clone())
            .app_data(api_ctx.clone())
            .app_data(app_settings.clone())
            .route("/", web::post().to(handlers::index))
    })
    .bind(ip.clone()).expect("В вашем провайдере отсутствует белый адрес.");

    println!("Server is running;");
    tokio::spawn(server.run());
    if settings.auto_connect {
        tools::set_webhook(&ip, &settings).await?;
    } else {
        loop {}
    }

    Ok(())
}

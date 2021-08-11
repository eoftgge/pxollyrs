use crate::events::PxollyResponse;
use crate::settings::Settings;
use anyhow::anyhow;

#[macro_export(local_inner_macros)]
macro_rules! params {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(params!(@single $rest)),*]));
    ($($key:expr => $value:expr,)+) => { params!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let cap = params!(@count $($key),*);
            let mut map = ::std::collections::HashMap::with_capacity(cap);
            $(
                let _ = map.insert($key, $value);
            )*
            map
        }
    };
}

pub fn vec_u64_to_string(vec: &Vec<u64>) -> String {
    vec.iter()
        .map(|u| u.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn parse_pxolly_response(result: PxollyResponse) -> String {
    match result {
        PxollyResponse::Success => 1,
        PxollyResponse::Fail => 0,
        PxollyResponse::UnknownErrorOrError => -1,
        PxollyResponse::UnknownUIDOrNoSupport => -2,
    }
    .to_string()
}

pub async fn confirmation_code(pxolly_token: &str) -> anyhow::Result<String> {
    let response = reqwest::Client::new()
        .post("https://api.pxolly.ru/method/callback.getSettings")
        .form(&params! {"access_token" => pxolly_token})
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(response
        .get("response")
        .ok_or(anyhow!("NoneError..."))?
        .get("confirmation_code")
        .ok_or(anyhow!("NoneError..."))?
        .as_str()
        .ok_or(anyhow!("NoneError..."))?
        .to_string())
}

pub async fn set_webhook(ip: &String, settings: &Settings) -> anyhow::Result<()> {
    let response = reqwest::Client::new()
        .post("https://api.pxolly.ru/method/callback.editSettings")
        .form(&params! {
            "url" => format!("http://{}", ip),
            "secret_key" => settings.secret_key.to_string(),
            "access_token" => settings.pxolly_token.to_string(),
        })
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    log::info!("Result connect to @pxolly: {:#?}", response);
    loop {}
}

pub async fn ip() -> anyhow::Result<String> {
    let response = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(response
        .get("origin")
        .ok_or(anyhow!("NoneError..."))?
        .as_str()
        .ok_or(anyhow!("NoneError..."))?
        .to_string())
}

pub struct Event {
    pub event: crate::events::PxollyEvent,
    pub api_ctx: crate::api_context::APIClient,
    pub chat: crate::chat_data::WorkChatData,
}

use crate::pxolly::api::PxollyAPI;
use crate::pxolly::types::categories::Categories;
use crate::pxolly::types::params::EditSettingsParams;

pub async fn auto_bind(api: PxollyAPI, is_bind: bool, secret_key: String, host: String, pxolly_url: String) {
    if is_bind || host == pxolly_url {
        return;
    }

    let response = api
        .callback()
        .edit_settings(EditSettingsParams {
            secret_key: Some(secret_key),
            url: Some(host),
            is_hidden: false,
        })
        .await;

    if let Ok(response) = response {
        log::info!("Result bind webhook: {:?}", response)
    } else if let Err(error) = response {
        log::error!("Failed bind webhook: {:?}", error)
    }
}
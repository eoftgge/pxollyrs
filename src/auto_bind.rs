use crate::pxolly::api::PxollyAPI;
use crate::pxolly::types::categories::Categories;
use crate::pxolly::types::params::EditSettingsParams;
use crate::pxolly::types::responses::callback::EditSettingsResponse;

pub async fn auto_bind(
    api: PxollyAPI,
    is_bind: bool,
    secret_key: String,
    host: String,
    pxolly_url: String,
) {
    if is_bind || host == pxolly_url {
        log::info!("auto_bind won't run.");
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

    match response {
        Ok(EditSettingsResponse { state: 200, .. }) => log::info!("Successfully bind webhook"),
        Ok(err) => log::error!("Failed bind webhook: {:?}", err),
        Err(err) => log::error!("Failed calling method @pxolly: {:?}", err),
    }
}

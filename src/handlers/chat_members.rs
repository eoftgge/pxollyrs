use crate::handlers::HandlerContext;
use crate::utils::models::PxollyResponse;
use crate::{par, PxollyResult};

pub async fn execute(ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
    let params = par! {
        "chat_id": ctx.peer_id - 2_000_000_000,
    };

    let response = ctx
        .client
        .api_request::<serde_json::Value>("messages.getChat", params)
        .await?;
    let users = response
        .get("users")
        .expect("Expect field: users")
        .as_array()
        .expect("Expect field: users_array");

    Ok(PxollyResponse::Text(
        users
            .iter()
            .map(|x| x.as_str().unwrap_or(""))
            .collect::<Vec<_>>()
            .join(","),
    ))
}

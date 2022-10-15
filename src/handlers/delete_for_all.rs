use serde_json::{Map, Value};

use crate::pxolly::context::HandlerContext;
use crate::pxolly::traits::TraitHandler;
use crate::pxolly::types::responses::PxollyResponse;
use crate::{par, PxollyResult};

pub struct DeleteForAll;

#[async_trait::async_trait]
impl TraitHandler for DeleteForAll {
    const EVENT_TYPE: &'static str = "delete_for_all";

    async fn execute(&self, ctx: HandlerContext) -> PxollyResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id(),
            "delete_for_all": 1,
            "cmids": ctx.object.conversation_message_ids
                .as_ref()
                .expect("Expect field: cmids")
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        };

        let response = ctx
            .client
            .api_request::<Map<String, Value>>("messages.delete", params)
            .await?;

        let mut cmids = Vec::new();
        for (id, is_success) in response.iter() {
            if is_success.as_u64().unwrap() != 0 {
                cmids.push(id.parse::<u64>().unwrap());
            }
        }

        return Ok(PxollyResponse::Text(
            cmids
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ));
    }
}

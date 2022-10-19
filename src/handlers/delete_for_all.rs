use super::prelude::*;
use serde_json::{Map, Value};

pub struct DeleteForAll {
    pub(crate) client: VKAPI,
}

#[async_trait::async_trait]
impl TraitHandler for DeleteForAll {
    const EVENT_TYPE: &'static str = "delete_for_all";

    async fn execute(&self, ctx: PxollyContext) -> PxollyResult<PxollyResponse> {
        let params = par! {
            "peer_id": ctx.peer_id()?,
            "delete_for_all": 1,
            "cmids": ctx.object.conversation_message_ids
                .as_ref()
                .expect("Expect field: cmids")
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        };

        let response = self
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

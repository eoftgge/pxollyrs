use crate::api::client::APIClient;
use crate::pxolly::types::events::PxollyEvent;
use crate::utils::config::SecretKey;
use crate::utils::database::DatabaseJSON;
use crate::utils::ConfirmationCode;
use std::sync::Arc;

pub struct HandlerContext {
    pub(crate) client: APIClient,
    pub(crate) database: Arc<DatabaseJSON>,
    pub(crate) event: PxollyEvent,
    pub(crate) code: ConfirmationCode,
    pub(crate) key: SecretKey,
    pub(crate) peer_id: Option<u64>,
}

impl HandlerContext {
    pub fn peer_id(&self) -> u64 {
        self.peer_id.expect("ohw. it's UB")
    }

    pub fn confirmation_code(&self) -> &str {
        &*self.code.0
    }

    pub fn secret_key(&self) -> &str {
        &*self.key.0
    }
}

impl std::ops::Deref for HandlerContext {
    type Target = PxollyEvent;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

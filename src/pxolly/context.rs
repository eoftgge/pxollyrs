use crate::errors::PxollyError;
use crate::pxolly::types::events::PxollyEvent;
use crate::pxolly::types::responses::PxollyResponse;
use crate::PxollyResult;

pub struct PxollyContext {
    event: PxollyEvent,
    peer_id: Option<u64>,
}

impl PxollyContext {
    pub fn new(event: PxollyEvent, peer_id: Option<u64>) -> Self {
        Self { event, peer_id }
    }

    pub fn peer_id(&self) -> PxollyResult<u64> {
        self.peer_id
            .ok_or(PxollyError::Response(PxollyResponse::ErrorCode(3)))
    }
}

impl std::ops::Deref for PxollyContext {
    type Target = PxollyEvent;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

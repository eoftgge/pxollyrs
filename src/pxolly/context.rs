use crate::pxolly::types::events::PxollyEvent;

pub struct HandlerContext {
    pub(crate) event: PxollyEvent,
    pub(crate) peer_id: Option<u64>,
}

impl HandlerContext {
    pub fn peer_id(&self) -> u64 {
        self.peer_id.expect("ohw. it's UB")
    }
}

impl std::ops::Deref for HandlerContext {
    type Target = PxollyEvent;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

use crate::pxolly::dispatch::handler::Handler;
use std::sync::Arc;

pub static mut EVENT_TYPES_HANDLERS: Vec<&'static str> = Vec::new();

pub struct Dispatcher<H: Handler, Tail: Clone> {
    pub(crate) handler: Arc<H>,
    pub(crate) tail: Tail,
}

impl<H: Handler, Tail: Clone> Clone for Dispatcher<H, Tail> {
    fn clone(&self) -> Self {
        Self {
            handler: Arc::clone(&self.handler),
            tail: self.tail.clone(),
        }
    }
}

#[derive(Clone)]
pub struct DispatcherBuilder;

pub trait PushHandler<NewHandler> {
    type Out;
    fn push_handler(self, handler: NewHandler) -> Self::Out;
}

impl<NewHandler: Handler> PushHandler<NewHandler> for DispatcherBuilder {
    type Out = Dispatcher<NewHandler, DispatcherBuilder>;

    fn push_handler(self, handler: NewHandler) -> Self::Out {
        Dispatcher {
            handler: Arc::new(handler),
            tail: DispatcherBuilder,
        }
    }
}

impl<H, Tail, NewHandler> PushHandler<NewHandler> for Dispatcher<H, Tail>
where
    Tail: PushHandler<NewHandler> + Clone,
    H: Handler,
    <Tail as PushHandler<NewHandler>>::Out: Clone,
{
    type Out = Dispatcher<H, <Tail as PushHandler<NewHandler>>::Out>;

    fn push_handler(self, handler: NewHandler) -> Self::Out {
        unsafe {
            EVENT_TYPES_HANDLERS.push(H::EVENT_TYPE);
        }
        Dispatcher {
            handler: self.handler,
            tail: self.tail.push_handler(handler),
        }
    }
}

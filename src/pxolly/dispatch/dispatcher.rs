use crate::pxolly::dispatch::traits::TraitHandler;
use std::sync::Arc;

pub static mut EVENT_TYPES_HANDLERS: Vec<&'static str> = Vec::new();

pub struct Dispatcher<Handler: TraitHandler, Tail: Clone> {
    pub(crate) handler: Arc<Handler>,
    pub(crate) tail: Tail,
}

impl<Handler: TraitHandler, Tail: Clone> Clone for Dispatcher<Handler, Tail> {
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

impl<NewHandler: TraitHandler> PushHandler<NewHandler> for DispatcherBuilder {
    type Out = Dispatcher<NewHandler, DispatcherBuilder>;

    fn push_handler(self, handler: NewHandler) -> Self::Out {
        Dispatcher {
            handler: Arc::new(handler),
            tail: DispatcherBuilder,
        }
    }
}

impl<Handler, Tail, NewHandler> PushHandler<NewHandler> for Dispatcher<Handler, Tail>
where
    Tail: PushHandler<NewHandler> + Clone,
    Handler: TraitHandler,
    <Tail as PushHandler<NewHandler>>::Out: Clone,
{
    type Out = Dispatcher<Handler, <Tail as PushHandler<NewHandler>>::Out>;

    fn push_handler(self, handler: NewHandler) -> Self::Out {
        unsafe {
            EVENT_TYPES_HANDLERS.push(Handler::EVENT_TYPE);
        }
        Dispatcher {
            handler: self.handler,
            tail: self.tail.push_handler(handler),
        }
    }
}

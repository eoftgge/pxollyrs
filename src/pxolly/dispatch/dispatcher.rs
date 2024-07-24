use crate::pxolly::dispatch::handler::Handler;
use std::sync::Arc;

pub struct Dispatcher<Current: Handler, Tail: Clone> {
    pub(crate) current: Arc<Current>,
    pub(crate) tail: Tail,
}

impl<H: Handler, Tail: Clone> Clone for Dispatcher<H, Tail> {
    fn clone(&self) -> Self {
        Self {
            current: Arc::clone(&self.current),
            tail: self.tail.clone(),
        }
    }
}

#[derive(Clone)]
pub struct DispatcherBuilder;

pub trait ComposeHandler<In> {
    type Out;
    fn compose(self, handler: In) -> Self::Out;
}

impl<In: Handler> ComposeHandler<In> for DispatcherBuilder {
    type Out = Dispatcher<In, DispatcherBuilder>;

    fn compose(self, handler: In) -> Self::Out {
        let handler = Arc::new(handler);
        Dispatcher {
            current: Arc::new(handler),
            tail: self,
        }
    }
}

impl<Current, Tail, In> ComposeHandler<In> for Dispatcher<Current, Tail>
where
    Tail: ComposeHandler<In> + Clone,
    Current: Handler,
    <Tail as ComposeHandler<In>>::Out: Clone,
{
    type Out = Dispatcher<Current, Dispatcher<Current, Tail>>;

    fn compose(self, handler: In) -> Self::Out {
        Dispatcher {
            current: handler,
            tail: self.tail.compose(self.current),
        }
    }
}

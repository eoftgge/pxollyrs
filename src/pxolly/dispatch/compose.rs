use std::sync::Arc;
use crate::handlers::prelude::Handler;
use crate::pxolly::dispatch::dispatcher::{Dispatcher, DispatcherBuilder};

pub trait ComposeHandler<In> {
    type Out;
    fn compose(self, handler: In) -> Self::Out;
}

impl<In: Handler> ComposeHandler<In> for DispatcherBuilder {
    type Out = Dispatcher<In, DispatcherBuilder>;

    fn compose(self, handler: In) -> Self::Out {
        let handler = Arc::new(handler);
        Dispatcher {
            current: handler,
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

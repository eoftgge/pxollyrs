
use crate::pxolly::dispatch::dispatcher::{Dispatch, Dispatcher, DispatcherBuilder};
use crate::pxolly::dispatch::handler::Handler;

pub trait ComposeHandler<In> {
    type Out;
    fn compose(self, handler: In) -> Self::Out;
}

impl<In: Handler> ComposeHandler<In> for DispatcherBuilder {
    type Out = Dispatcher<In, DispatcherBuilder>;

    fn compose(self, handler: In) -> Self::Out {
        Dispatcher {
            current: handler,
            tail: self,
        }
    }
}

impl<Current, Tail, In> ComposeHandler<In> for Dispatcher<Current, Tail>
where
    Tail: ComposeHandler<Current, Out = Dispatcher<Current, Tail>> + Dispatch,
    Current: Handler,
    In: Handler,
{
    type Out = Dispatcher<In, Dispatcher<Current, Tail>>;

    fn compose(self, handler: In) -> Self::Out {
        Dispatcher {
            current: handler,
            tail: self.tail.compose(self.current),
        }
    }
}

use crate::pxolly::traits::TraitHandler;

pub struct Dispatcher<Handler: TraitHandler, Tail> {
    pub(crate) handler: Handler,
    pub(crate) tail: Tail,
}

pub struct DispatcherBuilder;

pub trait PushHandler<NewHandler> {
    type Out;
    fn push_handler(self, handler: NewHandler) -> Self::Out;
}

impl<NewHandler: TraitHandler> PushHandler<NewHandler> for DispatcherBuilder {
    type Out = Dispatcher<NewHandler, DispatcherBuilder>;

    fn push_handler(self, handler: NewHandler) -> Self::Out {
        Dispatcher {
            handler,
            tail: DispatcherBuilder,
        }
    }
}

impl<Handler, Tail, NewHandler> PushHandler<NewHandler> for Dispatcher<Handler, Tail>
where
    Tail: PushHandler<NewHandler>,
    Handler: TraitHandler,
{
    type Out = Dispatcher<Handler, <Tail as PushHandler<NewHandler>>::Out>;

    fn push_handler(self, handler: NewHandler) -> Self::Out {
        Dispatcher {
            handler: self.handler,
            tail: self.tail.push_handler(handler),
        }
    }
}

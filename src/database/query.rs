use crate::database::session::DatabaseSession;
use crate::PxollyResult;

pub trait Finder {
    fn find(self) -> Self;
}

pub trait Insert {
    fn insert(self) -> Self;
}

pub trait Queryable: Finder {
    fn exec(self, session: &DatabaseSession) -> QueryBuilder;
}

pub struct QueryBuilder {}

impl std::future::IntoFuture for QueryBuilder {
    type Output = PxollyResult<T>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {})
    }
}

pub mod codegen {
    macro_rules! generate_for_model {
        ($model:ident) => {
            impl Finder for $model {
                fn find()
            }
        };
    }
}

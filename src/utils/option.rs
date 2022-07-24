use crate::errors::{PxollyResult, PxollyError};

pub trait ExpectedField<T> {
	fn expect_field(self, field: &str) -> PxollyResult<T>;
}

impl<T> ExpectedField<T> for Option<T> {
	#[inline]
	fn expect_field(self, field: &str) -> PxollyResult<T> {
		match self {
			Some(some) => Ok(some),
			None => Err(PxollyError::Other(format!("expected field `{}`", field)))
		}
	}
}
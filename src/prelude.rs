pub use crate::error::Error;
pub use std::format as f;

pub type Result<R> = std::result::Result<R, Error>;

// Generic Wrapper struct for newtype pattern, mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

macro_rules! s {
	() => {
		String::new()
	};
	($x:expr $(,)?) => {
		ToString::to_string(&$x)
	};
}

pub(crate) use s;

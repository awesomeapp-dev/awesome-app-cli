pub use crate::error::Error;
pub use std::format as f;

pub type Result<R> = std::result::Result<R, Error>;

macro_rules! s {
	() => {
		String::new()
	};
	($x:expr $(,)?) => {
		ToString::to_string(&$x)
	};
}

pub(crate) use s;

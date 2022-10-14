//! Various From implementations with the W() new type pattern.
//!

use crate::prelude::*;

/// Translate a &'a Option<Vec<String>> to a Vec<&'a str>
impl<'a> From<W<&'a Option<Vec<String>>>> for Vec<&'a str> {
	fn from(val: W<&'a Option<Vec<String>>>) -> Self {
		val
			.0
			.as_ref()
			.map(|v| v.iter().map(String::as_ref).collect::<Vec<_>>())
			.unwrap_or_default()
	}
}

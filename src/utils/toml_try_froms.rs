use crate::prelude::*;
use toml::Value;

impl TryFrom<W<Value>> for String {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<String> {
		String::try_from(W(&val.0))
	}
}

impl TryFrom<W<&Value>> for String {
	type Error = Error;
	fn try_from(val: W<&Value>) -> Result<String> {
		val
			.0
			.as_str()
			.map(|v| v.to_string())
			.ok_or_else(|| Error::XintoNotOfType("String"))
	}
}

impl TryFrom<W<Value>> for bool {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<bool> {
		bool::try_from(W(&val.0))
	}
}

impl TryFrom<W<&Value>> for bool {
	type Error = Error;
	fn try_from(val: W<&Value>) -> Result<bool> {
		val.0.as_bool().ok_or_else(|| Error::XintoNotOfType("Boolean"))
	}
}

impl TryFrom<W<&Value>> for u64 {
	type Error = Error;
	fn try_from(val: W<&Value>) -> Result<u64> {
		val
			.0
			.as_integer()
			.map(|v| v as u64)
			.ok_or_else(|| Error::XintoNotOfType("Number"))
	}
}

impl TryFrom<W<Value>> for u64 {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<u64> {
		u64::try_from(W(&val.0))
	}
}

impl TryFrom<W<&Value>> for Vec<Value> {
	type Error = Error;
	fn try_from(val: W<&Value>) -> Result<Vec<Value>> {
		val
			.0
			.as_array()
			.map(|v| v.clone())
			.ok_or_else(|| Error::XintoNotOfType("Array"))
	}
}

impl TryFrom<W<Value>> for Vec<Value> {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<Vec<Value>> {
		<Vec<Value>>::try_from(W(&val.0))
	}
}

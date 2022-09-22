use super::{XAs, XInto, XTake, XTakeInto};
use crate::prelude::*;
use toml::Value;

impl XInto<String> for Value {
	fn x_into(self) -> Result<String> {
		(&self).x_as()
	}
}

impl XInto<String> for &Value {
	fn x_into(self) -> Result<String> {
		self
			.as_str()
			.map(|v| v.to_string())
			.ok_or_else(|| Error::XintoNotOfType("String"))
	}
}

impl XInto<bool> for Value {
	fn x_into(self) -> Result<bool> {
		(&self).x_as()
	}
}

impl XInto<bool> for &Value {
	fn x_into(self) -> Result<bool> {
		self.as_bool().ok_or_else(|| Error::XintoNotOfType("Boolean"))
	}
}

impl XInto<u64> for Value {
	fn x_into(self) -> Result<u64> {
		(&self).x_as()
	}
}

impl XInto<u64> for &Value {
	fn x_into(self) -> Result<u64> {
		self
			.as_integer()
			.map(|v| v as u64)
			.ok_or_else(|| Error::XintoNotOfType("Number"))
	}
}

impl XInto<Vec<Value>> for Value {
	fn x_into(self) -> Result<Vec<Value>> {
		(&self).x_as()
	}
}

impl XInto<Vec<Value>> for &Value {
	fn x_into(self) -> Result<Vec<Value>> {
		// Note: Can we avoid the clone here?
		self.as_array().map(|v| v.clone()).ok_or_else(|| Error::XintoNotOfType("Array"))
	}
}

// region:    --- XTake
impl XTakeInto<String> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<String>> {
		let v = self.get(k).map(|v| v.x_as::<String>());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeInto<bool> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<bool>> {
		let v = self.get(k).map(|v| v.x_as::<bool>());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeInto<u64> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<u64>> {
		let v = self.get(k).map(|v| v.x_as::<u64>());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeInto<Vec<Value>> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<Vec<Value>>> {
		let v = self.get(k).map(|v| v.x_as::<Vec<Value>>());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeInto<Vec<String>> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<Vec<String>>> {
		let values = self.x_take::<Vec<Value>>(k)?;

		let values = values.map(|vs| vs.into_iter().map(|v| v.x_as::<String>()).collect::<Result<Vec<String>>>());
		match values {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}
// endregion: --- XTake

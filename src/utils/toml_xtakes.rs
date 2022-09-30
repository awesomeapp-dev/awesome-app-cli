use super::{XTake, XTakeInto};
use crate::prelude::*;
use toml::Value;

// region:    --- XTake
impl XTakeInto<String> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<String>> {
		let v = self.get(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeInto<bool> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<bool>> {
		let v = self.get(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeInto<u64> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<u64>> {
		let v = self.get(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeInto<Vec<Value>> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<Vec<Value>>> {
		let v = self.get(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeInto<Vec<String>> for Value {
	fn x_take_into(&mut self, k: &str) -> Result<Option<Vec<String>>> {
		let values = self.x_take::<Vec<Value>>(k)?;

		let values = values.map(|vs| vs.into_iter().map(|v| W(v).try_into()).collect::<Result<Vec<String>>>());
		match values {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}
// endregion: --- XTake

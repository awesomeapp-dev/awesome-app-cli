use super::{XTake, XTakeImpl};
use crate::prelude::*;
use toml::Value;

// region:    --- XTake
impl XTakeImpl<String> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<String>> {
		let v = self.get(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeImpl<bool> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<bool>> {
		let v = self.get(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeImpl<u64> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<u64>> {
		let v = self.get(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeImpl<Vec<Value>> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<Vec<Value>>> {
		let v = self.get(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}

impl XTakeImpl<Vec<String>> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<Vec<String>>> {
		let values = self.x_take::<Vec<Value>>(k)?;

		let values = values.map(|vs| vs.into_iter().map(|v| W(v).try_into()).collect::<Result<Vec<String>>>());
		match values {
			None => Ok(None),
			Some(r) => Ok(Some(r?)),
		}
	}
}
// endregion: --- XTake

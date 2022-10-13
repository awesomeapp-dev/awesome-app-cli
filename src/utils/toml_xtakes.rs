use super::XTakeImpl;
use crate::prelude::*;
use toml::Value;

// region:    --- XTakeImpl
impl XTakeImpl<String> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<String>> {
		self.get(k).map(|v| W(v).try_into()).transpose()
	}
}

impl XTakeImpl<bool> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<bool>> {
		self.get(k).map(|v| W(v).try_into()).transpose()
	}
}

impl XTakeImpl<u64> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<u64>> {
		self.get(k).map(|v| W(v).try_into()).transpose()
	}
}

impl XTakeImpl<Vec<Value>> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<Vec<Value>>> {
		self.get(k).map(|v| W(v).try_into()).transpose()
	}
}

impl XTakeImpl<Vec<String>> for Value {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<Vec<String>>> {
		let values: Option<Vec<Value>> = XTakeImpl::x_take_impl(self, k)?;

		values
			.map(|vs| vs.into_iter().map(|v| W(v).try_into()).collect::<Result<Vec<String>>>())
			.transpose()
	}
}
// endregion: --- XTakeImpl

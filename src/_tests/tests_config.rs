use crate::config::AWESOME_TMPL;
use crate::utils::XAs;
use toml::Value;

use super::Config;

#[test]
fn test_config_parsing() -> anyhow::Result<()> {
	let config = toml::from_str::<Value>(AWESOME_TMPL)?;
	let config: Config = config.x_as::<Config>()?;
	let runners = config.dev_runners.unwrap();

	assert_eq!(runners.len(), 7, "number of runners");

	for runner in runners.iter() {
		assert!(!runner.cmd.is_empty())
	}

	Ok(())
}

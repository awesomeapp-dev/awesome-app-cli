use super::Config;
use crate::config::AWESOME_TMPL;
use toml::Value;

#[test]
fn test_config_parsing() -> anyhow::Result<()> {
	let config = toml::from_str::<Value>(AWESOME_TMPL)?;
	let config: Config = Config::try_from(config)?;
	let runners = config.dev_runners.unwrap();

	assert_eq!(runners.len(), 7, "number of runners");

	for runner in runners.iter() {
		assert!(!runner.cmd.is_empty())
	}

	Ok(())
}

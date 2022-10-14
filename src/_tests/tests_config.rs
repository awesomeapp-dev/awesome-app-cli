use anyhow::Context;

use super::Config;
use crate::config::AWESOME_TMPL;

#[test]
fn test_config_parsing() -> anyhow::Result<()> {
	// --- Exec
	let config = toml::from_str::<Config>(AWESOME_TMPL)?;

	let dev_runners = config.dev.and_then(|v| v.runners).context("Should have dev runners")?;

	// --- Checks
	// Number of dev runners.
	assert_eq!(dev_runners.len(), 7, "number of runners");

	// Second runner.
	let runner = dev_runners.get(1).unwrap(); // Should be the 'tauri_icons'.
	assert_eq!(runner.name, "tauri_icons");

	Ok(())
}

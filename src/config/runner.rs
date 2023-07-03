use crate::exec::spawn_and_wait;
use crate::prelude::*;
use serde_derive::Deserialize;
use std::path::Path;
use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
pub struct Runner {
	pub name: String,

	pub working_dir: Option<String>,
	pub cmd: String,
	pub args: Option<Vec<String>>,

	pub when: Option<When>,

	#[serde(default)]
	pub wait_before: u64,

	#[serde(default)]
	pub concurrent: bool,

	#[serde(default)]
	pub end_all_on_exit: bool,
}

#[derive(Debug, Deserialize)]
pub struct When {
	no_file_at: Option<String>,
}

// region:    --- Runner Impl
#[cfg(target_os = "windows")]
const NPM_CMD: &str = "npm.cmd";
#[cfg(not(target_os = "windows"))]
const NPM_CMD: &str = "npm";

impl Runner {
	pub async fn exec(&self) -> Result<Option<Child>> {
		// --- Process the wait_before.
		if self.wait_before > 0 {
			println!(
				"Waiting {}ms (from runner {}.wait_before property)",
				self.wait_before, self.name
			);
			sleep(Duration::from_millis(self.wait_before)).await;
		}

		// --- Compute the cmd name.
		// Note: Special handling of "npm" which on Windows must be called as "npm.cmd"
		// TODO: Needs to generalize this. Could be more downstream, on ProgramNotFound error.
		let cmd_str: &str = self.cmd.as_ref();
		let cmd_str = if cmd_str.starts_with("npm") && cmd_str != NPM_CMD {
			NPM_CMD
		} else {
			cmd_str
		};

		// --- Compute the cmd args and working dir
		let args: Vec<&str> = W(&self.args).into();
		let cwd = self.working_dir.as_ref().map(Path::new);

		// --- Execute the command
		if !self.concurrent {
			spawn_and_wait(cwd, cmd_str, args.as_slice(), true)?;
			Ok(None)
		}
		// start the concurrent mode and add it in the concurrent watch list.
		else {
			let mut cmd = Command::new(cmd_str);
			cmd.args(args);
			let child = cmd.spawn()?;
			Ok(Some(child))
		}
	}

	pub fn should_run(&self, root_dir: &Path) -> Result<ShouldRun> {
		let no_file_at = self.when.as_ref().and_then(|w| w.no_file_at.as_ref());

		if let Some(no_file_at) = no_file_at {
			let no_file = root_dir.join(no_file_at);
			if Path::exists(&no_file) {
				return Ok(ShouldRun::No(f!("Path '{no_file_at}' found.")));
			}
		}

		Ok(ShouldRun::Yes)
	}
}

pub enum ShouldRun {
	Yes,
	No(String), // reason
}
// endregion: --- Runner Impl

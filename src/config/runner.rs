use crate::exec::spawn_and_wait;
use crate::prelude::*;
use crate::utils::{XTake, XTakeVal};
use std::path::Path;
use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::time::sleep;
use toml::Value;

#[derive(Debug)]
pub struct Runner {
	pub name: String,
	pub working_dir: Option<String>,
	pub cmd: String,
	pub args: Option<Vec<String>>,
	pub wait_before: u64,      // default to 0
	pub concurrent: bool,      // default to false
	pub end_all_on_exit: bool, // default to false
}

impl TryFrom<Value> for Runner {
	type Error = Error;
	fn try_from(mut val: Value) -> Result<Runner> {
		let name = val.x_take_val::<String>("name")?;
		let working_dir = val.x_take::<String>("working_dir")?;
		let cmd = val.x_take_val::<String>("cmd")?;
		let args = val.x_take::<Vec<String>>("args")?;
		let wait_before = val.x_take::<u64>("wait_before")?.unwrap_or(0);
		let concurrent = val.x_take::<bool>("concurrent")?.unwrap_or(false);
		let end_all_on_exit = val.x_take::<bool>("end_all_on_exit")?.unwrap_or(false);

		// TODO: Error when concurrent = false, and end_all_on_exit = true (would not make much sense)

		Ok(Runner {
			name,
			working_dir,
			wait_before,
			cmd,
			args,
			concurrent,
			end_all_on_exit,
		})
	}
}

// region:    --- Executor
#[cfg(target_os = "windows")]
const NPM_CMD: &str = "npm.cmd";
#[cfg(not(target_os = "windows"))]
const NPM_CMD: &str = "npm";

impl Runner {
	pub async fn exec(&self) -> Result<Option<Child>> {
		let name = &self.name;

		println!("==== Running runner: {name}");

		// --- Process the wait_before.
		if self.wait_before > 0 {
			println!(
				"Waiting {}ms (from runner {name}.wait_before property)",
				self.wait_before
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
			let mut cmd = Command::new(&cmd_str);
			cmd.args(args);
			let child = cmd.spawn()?;
			Ok(Some(child))
		}
	}
}
// endregion: --- Executor

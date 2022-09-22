use crate::config::ensure_config;
use crate::exec::spawn_and_wait;
use crate::prelude::*;
use crate::utils::XAs;
use clap::ArgMatches;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use sysinfo::{Pid, PidExt, Process, ProcessExt, ProcessRefreshKind, System, SystemExt};
use tokio::process::{Child, Command};
use tokio::time::sleep;

const WATCH_CHILD_DELAY: u64 = 3000; // in ms

#[cfg(target_os = "windows")]
const NPM_CMD: &str = "npm.cmd";
#[cfg(not(target_os = "windows"))]
const NPM_CMD: &str = "npm";

#[tokio::main]
pub async fn run_dev(_sub_cmd: &ArgMatches) -> Result<()> {
	// TODO: needs to get it from the params
	let root_dir = Path::new(".");

	let config = ensure_config(root_dir)?;

	// (runner_name, child, end_all_on_exit)
	let mut concurrent_children: Vec<(String, Child, bool)> = Vec::new();

	if let Some(runners) = config.dev_runners {
		for runner in runners.into_iter() {
			let name = &runner.name;

			println!("==== Running runner: {name}");

			if runner.wait_before > 0 {
				println!("Waiting {}ms (from runner {name}.wait_before property)", runner.wait_before);
				sleep(Duration::from_millis(runner.wait_before)).await;
			}

			let cmd_str: &str = runner.cmd.as_ref();

			// TODO: Needs to generalize this. Could be more downstream, on ProgramNotFound error.
			let cmd_str = if cmd_str.starts_with("npm") && cmd_str != NPM_CMD {
				NPM_CMD
			} else {
				cmd_str
			};

			let args = runner.args.unwrap_or_else(|| Vec::new());
			let args = args.iter().map(|v| v as &str).collect::<Vec<&str>>();

			let cwd = runner.working_dir.as_ref().map(|v| Path::new(v));

			if runner.concurrent == false {
				spawn_and_wait(cwd, &cmd_str, args.as_slice(), true)?;
			}
			// start the concurrent mode and add it in the concurrent watch list
			else {
				let mut cmd = Command::new(&cmd_str);
				cmd.args(args);
				let child = cmd.spawn()?;
				concurrent_children.push((name.to_string(), child, runner.end_all_on_exit));
			}
		}
	}

	// TODO: Probably need to change that to avoid doing those pollings.
	//       Might be a different strategy for nix v.s. win.
	if concurrent_children.len() > 0 {
		let mut end_all = false;

		let mut sys = System::new();

		'main: loop {
			// --- Check if any children is down
			for (_, child, end_flag) in concurrent_children.iter_mut() {
				let status = child.try_wait()?;
				if let Some(_) = status {
					if *end_flag {
						end_all = true;
					}
				}
			}

			// --- If end_all true, then, we terminate all
			if end_all {
				for (name, child, _) in concurrent_children.iter_mut() {
					if let None = child.try_wait()? {
						terminate_process_and_children(&mut sys, &name, child).await?
					}
				}
				break 'main;
			}

			sleep(Duration::from_millis(WATCH_CHILD_DELAY)).await;
		}
	}

	Ok(())
}

// NOTE: For now just one level down.
async fn terminate_process_and_children(sys: &mut System, name: &str, proc: &mut Child) -> Result<()> {
	if let Some(proc_id) = proc.id() {
		let proc_pid = Pid::from_u32(proc_id);

		// --- Fetch the children
		sys.refresh_processes_specifics(ProcessRefreshKind::everything().without_cpu());
		let processes = sys.processes();
		let children = find_descendant(processes, &proc_pid);

		// --- Terminate the parent
		match proc.kill().await {
			Ok(_) => (),
			Err(ex) => println!("Warning - error while stopping runner {name}. Cause: {ex}"),
		};

		// --- Terminate the children
		for (pid, name) in children {
			if let Some(process) = sys.process(pid.clone()) {
				let del = process.kill();
			}
		}
	}

	Ok(())
}

fn find_descendant(processes: &HashMap<Pid, Process>, root_pid: &Pid) -> Vec<(Pid, String)> {
	let mut children: HashMap<Pid, String> = HashMap::new();

	// NOTE: For now, probably going a little brute force, but this should be exhaustive
	//       and does not really have significant performance impact for the usecase.
	'main: loop {
		let mut cycle_has = false;
		for (pid, p) in processes.iter() {
			if let Some(parent_pid) = p.parent() {
				if !children.contains_key(pid) && (parent_pid == *root_pid || children.contains_key(&parent_pid)) {
					children.insert(pid.clone(), p.name().to_string());
					cycle_has = true;
				}
			}
		}
		// if this cycle did not find anything, we can break the search.
		if !cycle_has {
			break 'main;
		}
	}

	children.into_iter().collect()
}

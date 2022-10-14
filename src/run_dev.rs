use crate::config::ensure_awesome_toml;
use crate::prelude::*;
use clap::ArgMatches;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use sysinfo::{Pid, PidExt, Process, ProcessExt, ProcessRefreshKind, System, SystemExt};
use tokio::process::Child;
use tokio::time::sleep;

const WATCH_CHILD_DELAY: u64 = 3000; // in ms

#[tokio::main]
pub async fn run_dev(_sub_cmd: &ArgMatches) -> Result<()> {
	// TODO: needs to get it from the params.
	let root_dir = Path::new(".");

	// Read or create/read the Awesome.toml config with the dev runners.
	let config = ensure_awesome_toml(root_dir)?;

	// Vec to keep track of the concurrent processes.
	struct RunnerConcurrentSpawn {
		name: String,
		child: Child,
		end_all_on_exit: bool,
	}
	let mut children_to_watch: Vec<RunnerConcurrentSpawn> = Vec::new();

	// --- Exec each runner.
	if let Some(runners) = config.dev.and_then(|v| v.runners) {
		for runner in runners.iter() {
			// exec the runner.
			// returns a child if process is concurrent.
			let child = runner.exec().await?;

			// if concurrent, store for tracking.
			if let Some(child) = child {
				children_to_watch.push(RunnerConcurrentSpawn {
					name: runner.name.to_string(),
					child,
					end_all_on_exit: runner.end_all_on_exit,
				});
			}
		}
	}

	// --- Watch processes when concurrent to end_all_on_exit when flagged.
	// TODO: Probably need to change that to avoid doing polling.
	//       Strategy: Tokio Spawn for the child with mpsc for the end_all event.
	if !children_to_watch.is_empty() {
		let mut end_all = false;

		let mut sys = System::new();

		'main: loop {
			// --- Check if any children is down.
			for RunnerConcurrentSpawn {
				child, end_all_on_exit, ..
			} in children_to_watch.iter_mut()
			{
				let status = child.try_wait()?;
				if status.is_some() && *end_all_on_exit {
					end_all = true;
				}
			}

			// --- If end_all true, then, we terminate all.
			if end_all {
				for RunnerConcurrentSpawn { name, child, .. } in children_to_watch.iter_mut() {
					if (child.try_wait()?).is_none() {
						terminate_process_and_children(&mut sys, name, child).await?
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
		for (pid, _) in children {
			if let Some(process) = sys.process(pid) {
				let _ = process.kill();
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
					children.insert(*pid, p.name().to_string());
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

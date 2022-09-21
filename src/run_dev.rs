use crate::exec::exec_cmd_args;
use crate::prelude::*;
use clap::ArgMatches;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn run_dev(_sub_cmd: &ArgMatches) -> Result<()> {
	println!("\n=== Installing node packages");
	exec_cmd_args(None, "npm", &["install"], true)?;

	println!("\n=== Build src-tauri");
	let rust_dir = Path::new("src-tauri");
	exec_cmd_args(Some(rust_dir), "cargo", &["build"], true)?;

	// Starts the UI watch
	let wui_handle = thread::spawn(move || {
		println!("\n=== Build & Watch UI (.ts,.pcss) code.");
		match exec_cmd_args(None, "npm", &["run", "ui-dev"], true) {
			Ok(_) => (),
			Err(ex) => println!("\nERROR while Building & Watching UI - {ex}\n"),
		}
	});

	// Give some time to the build ui
	thread::sleep(Duration::from_millis(1000));

	// Starts the rust watch
	let tauri_handle = thread::spawn(move || {
		println!("\n=== Build & Watch Tauri (.rs) code.");
		match exec_cmd_args(None, "npm", &["run", "tauri", "dev"], true) {
			Ok(_) => (),
			Err(ex) => println!("\nERROR while Building & Watching Tauri - {ex}\n"),
		}
	});

	// TODO: Better handle wait.
	wui_handle.join().expect("wui_handle failed");
	tauri_handle.join().expect("tauri_handle failed");

	Ok(())
}

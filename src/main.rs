#![allow(unused)]
use crate::prelude::*;
use cmd_app::cmd_app;
use run_dev::run_dev;
use run_new::run_new;

mod cmd_app;
mod config;
mod error;
mod exec;
mod prelude;
mod run_dev;
mod run_new;
mod utils;

pub use cmd_app::VERSION;

fn main() {
	match cmd_run() {
		Ok(_) => (),
		Err(err) => println!("FAIL - {err}"),
	}
}

fn cmd_run() -> Result<()> {
	let app = cmd_app().get_matches();

	match app.subcommand() {
		Some(("new", sub_cmd)) => run_new(sub_cmd)?,
		Some(("dev", sub_cmd)) => run_dev(sub_cmd)?,
		_ => {
			// needs cmd_app version as the orginal got consumed by get_matches
			cmd_app().print_long_help()?;
			println!("\n");
		}
	}

	Ok(())
}

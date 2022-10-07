use clap::{crate_version, Arg, Command};

pub const VERSION: &str = crate_version!();

pub fn cmd_app() -> Command {
	Command::new("awesome-app")
		.version(VERSION)
		.about("Awesome Desktop App Scaffolder")
		.subcommand(sub_new())
		.subcommand(sub_dev())
}

fn sub_new() -> Command {
	Command::new("new")
		.about("Build new tauri app from template base")
		.arg(Arg::new("name").help("App/project name (no space)"))
}

fn sub_dev() -> Command {
	Command::new("dev").about("Starts hot-reload developement")
}

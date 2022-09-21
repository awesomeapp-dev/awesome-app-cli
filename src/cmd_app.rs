use clap::{crate_version, Arg, Command};

pub fn cmd_app() -> Command<'static> {
	Command::new("awesome-app")
		.version(&crate_version!()[..])
		.about("Awesome Desktop App Scaffolder")
		.subcommand(sub_new())
		.subcommand(sub_dev())
}

fn sub_new() -> Command<'static> {
	Command::new("new")
		.about("Build new tauri app from template base")
		.arg(Arg::new("name").help("App/project name (no space)"))
}

fn sub_dev() -> Command<'static> {
	Command::new("dev").about("Starts hot-reload developement")
}

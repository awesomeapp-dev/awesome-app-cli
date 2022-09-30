use crate::config::ensure_config;
use crate::exec::{prompt, spawn_and_wait, spawn_output};
use crate::prelude::*;
use crate::utils::{path_joins, safer_remove_dir_all};
use crate::Error;
use aho_corasick::AhoCorasick;
use clap::ArgMatches;
use regex::bytes::Regex;
use std::fs;
use std::path::Path;
use std::str::from_utf8;

const DEFAULT_APP_NAME: &str = "awesome-app";
const DEFAULT_WIN_TITLE: &str = "Awesome App";

// const FILE_PACKAGE: &str = "package.json";
// const FILE_TAURI_CONF: &str = "src-tauri/Tauri.toml";
// const FILE_VAPP: &str = "src-ui/src/views/v-app.ts";

const FILES: &[&str; 3] = &["package.json", "src-tauri/Tauri.toml", "src-ui/src/view/app-v.ts"];

const GIT_DIR: &str = ".git";

const GIT_TMPL_BASE: &'static str = "https://github.com/rust-awesome-app/template-app-base.git";
// const GIT_TMPL_MIN: &'static str = "https://github.com/jeremychone/rust-awesome-app-template-min.git";

struct Conf<'a> {
	app_name: &'a str,
	win_title: &'a str,
}

pub fn run_new(sub_cmd: &ArgMatches) -> Result<()> {
	check_git()?;

	// --- Get the name
	let app_name = sub_cmd.get_one::<String>("name");

	let app_name = match app_name {
		Some(name) => name.to_string(),
		None => prompt(&f!("What is your app name? ({DEFAULT_APP_NAME}): "), Some(DEFAULT_APP_NAME))?,
	};

	// --- Get the title
	let app_title = prompt(&f!("What should the window title be? ({app_name}): "), Some(&app_name))?;

	// --- Compute the app dir
	let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
	let app_dir_name = re.replace_all(app_name.as_bytes(), "-".as_bytes());
	// remove the last '-' chars
	let app_dir_name = Regex::new(r"[-]+$")
		.unwrap()
		.replace_all(&app_dir_name, "".as_bytes())
		.into_owned();
	let app_dir_name = from_utf8(&app_dir_name).unwrap().to_lowercase();
	let app_dir = Path::new(&app_dir_name);

	// check if the dir already exist
	if app_dir.exists() {
		return Err(Error::DirAlreadyExist(s!(app_dir.to_string_lossy())));
	}

	// --- Do the git clone
	println!("Cloning rust-awesome- (base app template)");
	// git clone --depth 1 --branch <tag_name> <repo_url>
	spawn_and_wait(
		None,
		"git",
		&["clone", "--depth", "1", "--branch", "main", GIT_TMPL_BASE, &app_dir_name],
		true,
	)?;

	// --- Replace the parts
	replace_parts(
		app_dir,
		Conf {
			app_name: &app_name,
			win_title: &app_title,
		},
	)?;

	// --- Add the Awesome.toml
	ensure_config(app_dir)?;

	// --- Remove the git folder
	let git_dir = app_dir.join(GIT_DIR);
	println!("Delete template git directory ({})", git_dir.to_string_lossy());
	safer_remove_dir_all(&git_dir)?;

	// --- Do the git init and initial
	spawn_and_wait(Some(app_dir), "git", &["init", "."], true)?;
	spawn_and_wait(Some(app_dir), "git", &["add", "-A", "."], true)?;
	spawn_and_wait(Some(app_dir), "git", &["commit", "-a", "-m", ". initial"], true)?;

	println!(
		"
Next steps:

> cd {app_dir_name}

> awesome-app dev

First compile takes a little while, but then, hot-reload dev !!! 

Open the {app_dir_name} in your IDE.

Happy coding!
"
	);

	Ok(())
}

fn replace_parts(dir: &Path, conf: Conf) -> Result<()> {
	let files = FILES.into_iter().map(|f| path_joins(dir, f)).collect::<Vec<_>>();

	let patterns = &[DEFAULT_APP_NAME, DEFAULT_WIN_TITLE];
	let ac = AhoCorasick::new(patterns);
	let replace_by = &[conf.app_name, conf.win_title];

	for file in files {
		let content = fs::read_to_string(&file)?;
		let res = ac.replace_all_bytes(content.as_bytes(), replace_by);
		let new_content = std::str::from_utf8(&res).unwrap();

		if content != new_content {
			println!("File updated: '{}'", file.to_string_lossy());
			fs::write(file, new_content)?;
		} else {
			println!("File skipped (nothing changed): '{}'", file.to_string_lossy());
		}
	}

	Ok(())
}

// region:    --- Utils

fn check_git() -> Result<()> {
	spawn_output(None, "git", &["--version"], false).or_else(|_| Err(Error::GitNotPresent))?;
	Ok(())
}
// endregion: --- Utils

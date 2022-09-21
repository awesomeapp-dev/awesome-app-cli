use crate::prelude::*;
use std::io::{self, stdin, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

pub fn prompt(message: &str, default: Option<&str>) -> Result<String> {
	print!("{message}");
	let _ = io::stdout().flush();

	let mut buf = String::new();
	stdin().read_line(&mut buf).or_else(|_| Err(Error::StdinFailToReadLine))?;

	let val = buf.trim();

	let val = match (val.is_empty(), default) {
		(true, Some(default)) => default,
		(false, _) => val,
		(true, None) => val, // return the empty string (TODO: might want to return error)
	};

	Ok(val.to_string())
}

pub fn exec_proc(proc: &mut Command) -> Result<ExitStatus> {
	Ok(proc.spawn()?.wait()?)
}

pub fn exec_cmd_args(cwd: Option<&Path>, cmd: &str, args: &[&str], print_exec: bool) -> Result<()> {
	let mut proc = Command::new(cmd);
	if let Some(cwd) = cwd {
		proc.current_dir(cwd);
	}
	proc.args(args);

	if print_exec {
		println!("> executing: {} {}", cmd, args.join(" "));
	}

	match exec_proc(&mut proc) {
		Ok(status) => {
			if !status.success() {
				Err((cmd, args, status).into())
			} else {
				Ok(())
			}
		}
		Err(ex) => Err(ex),
	}
}

pub fn exec_to_stdout(cwd: Option<&PathBuf>, cmd: &str, args: &[&str], print_exec: bool) -> Result<String> {
	if print_exec {
		println!("> executing: {} {}", cmd, args.join(" "));
	}
	let mut proc = Command::new(&cmd);
	if let Some(cwd) = cwd {
		proc.current_dir(cwd);
	}
	proc.args(args);
	match proc.stdout(Stdio::piped()).output() {
		Err(ex) => Err(ex.into()),
		Ok(output) => {
			let txt = if output.status.success() {
				String::from_utf8(output.stdout)
			} else {
				String::from_utf8(output.stderr)
			};

			match txt {
				Err(ex) => Err(Error::ExecError(s!(cmd), f!("{ex:?}"))),
				Ok(txt) => Ok(txt),
			}
		}
	}
}

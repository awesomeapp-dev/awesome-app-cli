use std::process::ExitStatus;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Fail to execute {0} cause: {1}")]
	ExecError(String, String),

	#[error("Path not safe to delete {0}")]
	PathNotSafeToDelete(String),

	#[error("Directory {0} already exist. Cancelling.")]
	DirAlreadyExist(String),

	#[error("git command line not found. Required for awesome-app.")]
	GitNotPresent,

	#[error("Fail to read line")]
	StdinFailToReadLine,

	#[error(transparent)]
	IOError(#[from] std::io::Error),
}

type ExecWithExitStatus<'a> = (&'a str, &'a [&'a str], ExitStatus);

impl<'a> From<ExecWithExitStatus<'a>> for Error {
	fn from(val: ExecWithExitStatus) -> Self {
		Error::ExecError(val.0.to_string(), "".to_string())
	}
}

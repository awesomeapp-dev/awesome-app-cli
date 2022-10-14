use crate::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

mod froms;
mod toml_try_froms;
mod toml_xtakes;
mod x_take;

// --- re-exports
pub use x_take::*;

// region:    --- File & Path Utils
const DELETE_MUST_CONTAINS_ANY_OF: &[&str; 1] = &[".git"];

/// Rudementary hardcoded way to check if a path seems to be safe to be deleted.
/// Designed to be application specific.
/// (far from perfect, but better than nothing)
pub fn safer_remove_dir_all(dir: &Path) -> Result<()> {
	let dir_str = dir.to_string_lossy();

	DELETE_MUST_CONTAINS_ANY_OF
		.iter()
		.find(|v| dir_str.contains(*v))
		.ok_or_else(|| Error::PathNotSafeToDelete(s!(dir_str)))?;

	// TODO: Make error more informative
	fs::remove_dir_all(dir)?;
	Ok(())
}

/// Create a new PathBuf from a root and a '/' delimited components
pub fn path_joins(root: &Path, sub_path: &str) -> PathBuf {
	let parts = sub_path.split('/');
	let mut path = root.to_owned();
	for part in parts {
		path.push(part)
	}
	path
}

// endregion: --- File & Path Utils

use crate::run_new::clear_awesome_toml_from_gitignore;

const TEST_GITIGNORE: &str = "
# --- tauri dist build assets
dist/js/
dist/css/

# --- Awesome.toml
# From the template app code, Awesome.toml is ignored as this should be generated 
# by 'awesome-app new` or 'awesome-app dev' if not present.
#
# However, in full application code, this could be committed (and line below commented), as it could be changed per project.
Awesome.toml

# --- No png except in dist or the app-icon.
# For generating the app icons, npm run tauri icon src-tauri/icons/app-icon.png)
*.png
";

#[test]
fn test_gitignore_remove_awesome() -> anyhow::Result<()> {
	let result = clear_awesome_toml_from_gitignore(TEST_GITIGNORE);

	assert!(!result.contains("Awesome.toml"), "should not container Awesome.toml");

	Ok(())
}


#[cfg(feature = "cli")] 
#[path = ""]
mod reexport_cli_features {
    use minijinja::Environment;
    use serde::Serialize;
    use crate::Args;
}
#[cfg(feature = "cli")]
pub use reexport_cli_features::*;

/// new_project
///
/// Generates the config file if it doesn't already exist.
/// Verifies a crate is reachable using $CARGO_MANIFEST_DIR.
pub fn new_project(name: &str) -> Result<(), 'static str> {
    // todo: Return `Err` if project already exists
    let path = "data/base.toml";
    let mut env = Environment::new();
    env.get_template(&path).unwrap();
    Ok(())
}




/*
#[cfg(feature = "cli")] 
#[path = ""]
mod config_reexport_modules {
    mod serde_derive;
    mod toml;

}
*/

/// new_project
///
/// Generates the config file if it doesn't already exist.
/// Verifies a crate is reachable using $CARGO_MANIFEST_DIR.
pub fn new_project(name: &str) -> Result<(), 'static str> {
    todo!()
}

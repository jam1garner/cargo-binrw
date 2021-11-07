use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "cli")] {
        //use std::env;
        //use std::net::IpAddr;
        //use std::str::FromStr;
        //use std::path::{PathBuf};
        use structopt::StructOpt;
        use minijinja::Environment;
        use serde::Serialize;
        use crate::Args;
    }
}

#[cfg(feature = "cli")]
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




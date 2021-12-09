use crate::Error;

use std::fs;
use std::io::Cursor;
use std::convert::TryInto;
use std::path::{Path, PathBuf};
use std::process::{Stdio, Command};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn get_cargo_dir() -> PathBuf {
    dirs::home_dir()
        .expect("No home directory found")
        .push_join(".cargo")
        .ensure_exists()
}


fn get_cargo_binrw_dir() -> PathBuf {
    get_cargo_dir()
        .push_join("skyline")
        .ensure_exists()
}


fn get_binrw_toolchain_dir() -> PathBuf {
    get_cargo_binrw_dir()
        .push_join("toolchain")
        .ensure_exists()
}

fn get_toolchain() -> PathBuf {
    get_binrw_toolchain_dir()
        .push_join("binrw")
        .ensure_exists()
}

fn get_version_file() -> PathBuf {
    get_toolchain()
        .push_join("version")
}

fn get_current_version() -> Option<String> {
    fs::read_to_string(get_version_file().if_exists()?).ok()
}

fn rustup_toolchain_link(name: &str, path: &Path) -> Result<(), Error> {
    let status = Command::new("rustup")
        .args(&["toolchain", "link", name])
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| Error::RustupNotFound)?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::RustupLinkFailed)
    }
}

/*
   I'm not 100% sure if it's warranted to add the actual `update_std` function
   here, since it's a little out of scope, but it does seem handy to be able
   to call things that are relevant to the cargo toolchain for libtraceserver.so
*/

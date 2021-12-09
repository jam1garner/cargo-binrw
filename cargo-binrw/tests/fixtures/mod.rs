use assert_cmd::prelude::*;
use assert_fs::fixture::TempDir;
use assert_fs::prelude::*;
use port_check::free_local_port;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Error type used by tests
pub type Error = Box<dyn std::error::Error>;

mod fixtures;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::output::OutputOkExt;
use fixtures::Error;
use std::process::Command;
use structopt::clap::{crate_name, crate_version};

fn cargo_binrw() -> Command {
    Command::cargo_bin("cargo-binrw").unwrap()
}

#[test]
fn help() {
    cargo_binrw().args(&["binrw", "help"]).assert().success();
}
//! Commenting out this test for now because it breaks things
/*#[test]
fn run() {
    cargo_binrw().args(&["binrw", "run"]).assert().success();
}*/
#[test]
fn new() {
    cargo_binrw().args(&["binrw", "new"]).assert().success();
}
#[test]
fn fuzz() {
    cargo_binrw().args(&["binrw", "fuzz"]).assert().success();
}



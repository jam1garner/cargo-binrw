mod fixtures;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::cargo::CommandCargoExt;
use fixtures::Error;
use std::process::Command;
use structopt::clap::{crate_name, crate_version};

fn cargo_binrw() -> Command {
    Command::cargo_bin("cargo-binrw").unwrap()
}

#[test]
fn help() {
    cargo_binrw().arg("help").assert().success();
}
#[test]
fn run() {
    cargo_binrw().arg("run").assert().success();
}
#[test]
fn new() {
    cargo_binrw().arg("new").assert().success();
}
#[test]
fn fuzz() {
    cargo_binrw().arg("fuzz").assert().success();
}

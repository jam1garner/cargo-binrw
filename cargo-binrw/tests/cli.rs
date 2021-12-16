mod fixtures;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::output::OutputOkExt;
use fixtures::Error;
use std::process::{ Command, Stdio };
use core::time::Duration;
use structopt::clap::{crate_name, crate_version};
use wait_timeout::ChildExt;

fn cargo_binrw() -> Command {
    Command::cargo_bin("cargo-binrw").unwrap()
}

#[test]
fn help() {
    cargo_binrw().args(&["binrw", "help"]).assert().success();
}

#[test]
fn run() {
    use wait_timeout::ChildExt;
   

    let mut child = cargo_binrw().args(&["binrw", "run"]).stdout(Stdio::piped()).spawn().unwrap();
    
    let secs = Duration::from_secs(2);
    let _status_code = match child.wait_timeout(secs).unwrap() {
        Some(status) => status.code(),
        None => {
            child.kill().unwrap();
            child.wait().unwrap().code()
        }   
    };
}

#[test]
fn new() {
    cargo_binrw().args(&["binrw", "new", "."]).assert().success();
}

#[test]
fn new_fail() {
    cargo_binrw().args(&["binrw", "new"]).assert().failure();
}

#[test]
fn fuzz() {
    cargo_binrw().args(&["binrw", "fuzz"]).assert().success();
}


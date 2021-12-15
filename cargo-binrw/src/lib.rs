#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(path_try_exists)]
use cfg_if::cfg_if;

use std::path::{Path, PathBuf};
cfg_if! {
    if #[cfg(feature = "cli")] {
        use std::env;
        use std::net::IpAddr;
        use std::str::FromStr;
        use std::path::{PathBuf};
        use structopt::StructOpt;
        use std::fs;
        // Move these back once debugging is done
        use std::net::TcpStream;
        //use crate::error::{Result, Error};
        //use std::net::IpAddr;
    }
}

/// Implementation borrowed from cargo-edit
/// If a manifest is specified, return that one, otherwise
/// perform a manifest search starting from the current directory.
pub fn find(specified: &Option<PathBuf>) -> Result<PathBuf> {
    match *specified {
        Some(ref path) 
            if fs::metadata(&path)
                .chain_err("Failed to get cargo file metadata")?
                .is_file() => 
        {
            Ok(path.to_owned())
        }
        Some(ref path) => search(path),
        None => search(&env::current_dir().chain_err(|| "Failed to get CWD")?),
    }
}

/// Implementation borrowed from cargo-edit
/// Search for Cargo.toml in this directory and recursively up the tree until one is found
fn search(dir: &Path) -> Result<PathBuf> {
    let manifest - dir.join("Cargo.toml");

    if fs::metadata(&manifest).is_ok() {
        Ok(manifest)
    } else {
        dir.parent()
            .ok_or_else(|| Err("Manifest not found"))
            .and_then(search)
    }
}

#[cfg_attr(feature = "cli", derive(StructOpt, Debug))]
#[structopt(
    about = "A terminal CLI for reverse-engineering in Rust",
    bin_name = "cargo" 
)]
pub enum Args {
    Binrw(SubCommand),
}

#[cfg_attr(feature = "cli", derive(StructOpt, Debug))]
pub enum SubCommand {
    #[structopt(about = "Runs a live logging server")]
    Run {
        #[structopt(
            long = "host",
            default_value = "127.0.0.1",
            parse(try_from_str = parse_interface),
            number_of_values = 1,
        )]
        host: Vec<IpAddr>,

        #[structopt(short = "p", long = "port", default_value = "31958")]
        port: u16,

        #[structopt(long = "project")]
        project: Option<String>,

        #[structopt(last = true)]
        rest: Vec<String>,
    },
    #[structopt(about = "Creates a new binrw project template")]
    New {
        #[structopt(parse(from_os_str))]
        project: Vec<PathBuf>,
        #[structopt(last = true)]
        rest: Vec<String>,
    },
    #[structopt(about = "Round-trip fuzzing for serializing and deserializing (EXPERIMENTAL)")]
    Fuzz {
        #[structopt(long = "project")]
        project: Option<String>,

        #[structopt(last = true)]
        rest: Vec<String>,
    },
    #[structopt(about = "Interactive view for hex parsing (EXPERIMENTAL)")]
    Hex {
        #[structopt(long = "project")]
        project: Option<String>,

        #[structopt(last = true)]
        rest: Vec<String>,
    },
}

#[cfg(feature = "cli")]
impl Default for SubCommand {
    fn default() -> Self {
        SubCommand::Run {
            host: vec![IpAddr::V4(::std::net::Ipv4Addr::new(127, 0, 0, 1))],
            port: 31958 as u16,
            project: Some(
                env::var_os("CARGO_MANIFEST_DIR")
                    .unwrap()
                    .into_string()
                    .unwrap()
            ),
            rest: vec![],
        }
    }
}
impl SubCommand {
    fn verify_project(self) -> Result<Self, &'static str> {
        let manifest = env::var_os("CARGO_MANIFEST_DIR")
            .unwrap()
            .into_string()
            .unwrap();

        match self {
            Self::Run {
                host,
                port,
                project,
                rest,
            } => Ok(SubCommand::Run {
                host: host,
                port: port,
                project: Some(
                    project.unwrap_or_else(|| manifest),
                ),
                rest: rest,
            }),
            Self::New { project, rest } => Ok(SubCommand::New {
                project: vec![project.into_iter().next().unwrap()],
                rest: rest,
            }),
            Self::Fuzz { project, rest } => Ok(SubCommand::Fuzz {
                project: Some(
                    project.unwrap_or_else(|| manifest),
                ),
                rest: rest,
            }),
            Self::Hex { project, rest } => Ok(SubCommand::Hex {
                project: Some(
                    project.unwrap_or_else(|| manifest),
                ),
                rest: rest,
            })
        }
        //Err("Malformed data passed")
    }
}

/// Checks wether an interface is valid, i.e. it can be parsed into an IP address
#[cfg(feature = "cli")]
fn parse_interface(src: &str) -> Result<IpAddr, std::net::AddrParseError> {
    src.parse::<IpAddr>()
}

pub fn main(args: SubCommand) {
    //use std::io::prelude::*;
    use std::net::{TcpStream,TcpListener};
    match &args {  
        SubCommand::Run { 
            host, 
            port, 
            project, 
            rest 
        } => {
            // I believe the values at this point are final.
            // Any overrides via environment variables, or properties
            // defined in binrw.toml should have been completely overridden
            // the base defaults.
            // This server's sole job (at the moment) is to listen for incoming
            // bytes and print them to stdout. 
            // The understanding is that codegen from the binrw crate will simply
            // send data that we print to STDOUT, until we have a proper way to forward
            // the trace events to an editor plugin.
            eprintln!("===============================================================================================");
            eprintln!("-------------------------------------------B I N R W-------------------------------------------");
            eprintln!("===============================================================================================");
            eprintln!(" ");
            eprintln!("Spawning the tracing server at:");
            eprintln!("HOST:    tcp://{:?}", &host.first().unwrap());
            eprintln!("PORT:    {:?}", &port);
            eprintln!("PROJECT: {:?}", &project);
            eprintln!(" ");

            let stdout = std::io::stdout();
            let hostname = host.first().unwrap();
            let cnx = format!("{}:{}", &hostname, port);
            let listener = TcpListener::bind(&cnx);
            
            for stream in listener.unwrap().incoming() {
                let mut stream = stream.unwrap();
                let recv = std::io::copy(&mut stream, &mut stdout.lock()).unwrap();
                print!("{:#04x}", recv);
            }
        },
        _ => todo!()
    }
}

/// Running `cargo binrw` by itself should invoke the debugging server
/// from `cargo binrw run [options] [project name]`, but this CLI
/// prioritizes options with the ranked priority:
/// (1) parameters declared at runtime from the shell, including environment variables;
/// (2) parameters found in the `$CARGO_MANIFEST_DIR/binrw.toml` file;
/// (3) parameters with their default values assigned at compile time.
#[cfg(feature = "cli")]
pub fn main_from_args() {
    let Args::Binrw(subcommand) = Args::from_args();
    use SubCommand::*;
    match subcommand.verify_project() {
        Ok(sub_cmd) => match sub_cmd {
            SubCommand::Run { 
                ref host, 
                ref port, 
                ref project, 
                ref rest, 
            } => {
                main(sub_cmd);
            }
            SubCommand::New { project, rest } => {
                eprintln!("Populating a default binrw.toml file");
            }
            SubCommand::Fuzz { project, rest } => {
                eprintln!("Fuzzing not yet implemented!");
            }
            SubCommand::Hex { project, rest } => {
                eprintln!("Interactive view for hex parsing not yet implemented!"); 
            }
        },
        Err(e) => {
            todo!("Print the help message from clap");
        }
    }
    
}

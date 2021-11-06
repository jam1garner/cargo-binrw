#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(feature = "cli")]
use std::net::IpAddr;

#[cfg(feature = "cli")]
use std::str::FromStr;

#[cfg(feature = "cli")]
use std::path::PathBuf;

#[cfg(feature = "cli")]
use structopt::StructOpt;

#[cfg_attr(feature = "cli", derive(StructOpt, Debug))]
#[structopt(
    about = "A terminal CLI for reverse-engineering in Rust",
    bin_name = "cargo binrw"
)]
pub enum Args {
    #[structopt(about = "Runs a live logging server")]
    Run {
        #[structopt(
            short = "h",
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
impl Default for Args {
    fn default() -> Self {
        Args::Run {
            host: vec![IpAddr::V4(::std::net::Ipv4Addr::new(127, 0, 0, 1))],
            port: 42069 as u16,
            project: Some(::std::env::var("CARGO_MANIFEST_DIR").unwrap()),
            rest: vec![],
        }
    }
}
impl Args {
    fn verify_project(self) -> Self {
        match self {
            Self::Run {
                host,
                port,
                project,
                rest,
            } => Args::Run {
                host: host,
                port: port,
                project: Some(
                    project.unwrap_or_else(|| ::std::env::var("CARGO_MANIFEST_DIR").unwrap()),
                ),
                rest: rest,
            },
            Self::New { project, rest } => Args::New {
                project: vec![project.into_iter().next().unwrap()],
                rest: rest,
            },
            Self::Fuzz { project, rest } => Args::Fuzz {
                project: Some(
                    project.unwrap_or_else(|| ::std::env::var("CARGO_MANIFEST_DIR").unwrap()),
                ),
                rest: rest,
            },
            Self::Hex { project, rest } => Args::Hex {
                project: Some(
                    project.unwrap_or_else(|| ::std::env::var("CARGO_MANIFEST_DIR").unwrap()),
                ),
                rest: rest,
            },
        }
    }
}

/// Checks wether an interface is valid, i.e. it can be parsed into an IP address
#[cfg(feature = "cli")]
fn parse_interface(src: &str) -> Result<IpAddr, std::net::AddrParseError> {
    src.parse::<IpAddr>()
}

pub fn main(args: Args) {
    eprintln!("{:?}", &args);
}

/// Running `cargo binrw` by itself should invoke the debugging server
/// from `cargo binrw run [options] [project name]`, but this CLI
/// prioritizes options with the ranked priority:
/// (1) parameters declared at runtime from the shell, including environment variables;
/// (2) parameters found in the `$CARGO_MANIFEST_DIR/binrw.toml` file;
/// (3) parameters with their default values assigned at compile time.
#[cfg(feature = "cli")]
pub fn main_from_args() {
    
    let properties = if ::std::env::args().len() == 1 as usize {
        // Check whether `$CARGO_MANIFEST_DIR/binrw.toml` exists. If it does, load it into
        // the runtime and override any default values with those specified in this file.
        todo!();
    } else {
        // Otherwise use the default, out-of-the-box values
        todo!();
    };

    let args = StructOpt::from_args();
    main(args)
}

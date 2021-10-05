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
    bin_name = "cargo binrw",
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

        #[structopt(short = "p", long = "port", default_value = "42069")]
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
                rest: vec![]
            }
    }
}
impl Args {
    fn verify_project(&mut self) -> Self {
        
        match self.as_ref() {
            &mut Self::Run {host, port, project, rest } => { 
                Args::Run { 
                    host: host,
                    port: port,
                    project: Some(project.unwrap_or_else(|| ::std::env::var("CARGO_MANIFEST_DIR").unwrap())),
                    rest: rest
                }
            },
            &mut Self::New { project, rest } => { 
                Args::New {
                    project: vec![project.into_iter().next().unwrap()],
                    rest: rest
                }
            },
            &mut Self::Fuzz { project, rest } => { 
                Args::Fuzz {
                    project: Some(project.unwrap_or_else(|| ::std::env::var("CARGO_MANIFEST_DIR").unwrap())),
                    rest: rest
                }
            },
            &mut Self::Hex { project, rest } => {
                Args::Hex {
                    project: Some(project.unwrap_or_else(|| ::std::env::var("CARGO_MANIFEST_DIR").unwrap())),
                    rest: rest
                }
            }
        }
    }
}

/// Checks wether an interface is valid, i.e. it can be parsed into an IP address
#[cfg(feature = "cli")]
fn parse_interface(src: &str) -> Result<IpAddr, std::net::AddrParseError> {
    src.parse::<IpAddr>()
}

/*
#[cfg(feature = "cli")]
#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum CargoArgsWrapper {
    Binrw(Args),
}
*/
pub fn main(args: Args) {
    eprintln!("{:?}", &args);
    
}

#[cfg(feature = "cli")]
pub fn main_from_args() {
    //let CargoArgsWrapper::Binrw(args) = StructOpt::from_args();
    for (idx, arg) in ::std::env::args().enumerate() {
        println!("{}: {}", idx, arg);
    }
    //let args = StructOpt::from_args();
    //main(args)
}

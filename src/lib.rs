#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(path_try_exists)]
use std::path::Path;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "cli")] {
        use std::env;
        use std::net::IpAddr;
        use std::str::FromStr;
        use std::path::{PathBuf};
        use structopt::StructOpt;
    }
}


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
            port: 31958 as u16,
            project: Some(env::var_os("CARGO_MANIFEST_DIR").unwrap().into_string().unwrap()),
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
                    project.unwrap_or_else(|| env::var_os("CARGO_MANIFEST_DIR").unwrap().into_string().unwrap()),
                ),
                rest: rest,
            },
            Self::New { project, rest } => Args::New {
                project: vec![project.into_iter().next().unwrap()],
                rest: rest,
            },
            Self::Fuzz { project, rest } => Args::Fuzz {
                project: Some(
                    project.unwrap_or_else(|| env::var_os("CARGO_MANIFEST_DIR").unwrap().into_string().unwrap()),
                ),
                rest: rest,
            },
            Self::Hex { project, rest } => Args::Hex {
                project: Some(
                    project.unwrap_or_else(|| env::var_os("CARGO_MANIFEST_DIR").unwrap().into_string().unwrap()),
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
 
        let mut content = String::from(include_str!("../data/default.toml").to_string());
        
        if let Some(manifest_dir) = ::std::env::var_os("CARGO_MANIFEST_DIR") {
            
            let some_propfile = {
                match Path::new(&manifest_dir).join("binrw.toml") {
                    project => {
                        if project.try_exists().unwrap() { Some(project) } 
                        else { None }
                    },
                    
                }    
            };
            match some_propfile {
                Some(toml_path) => {
                    *&mut content = String::from(::std::fs::read_to_string(toml_path)
                                                 .expect("Please report this as a bug."));
                },
                None => {
                    // Default behavior handles this case
                }
            };
        };
    };

    let args = StructOpt::from_args();
    main(args)
}

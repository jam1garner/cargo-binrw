#[cfg(feature = "cli")]
use structopt::StructOpt;

#[cfg_attr(feature = "cli", derive(StructOpt))]
pub struct Args {
    // TODO
}

#[cfg(feature = "cli")]
#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum CargoArgsWrapper {
    Binrw(Args),
}

pub fn main(args: Args) {
    todo!()
}

#[cfg(feature = "cli")]
pub fn main_from_args() {
    let CargoArgsWrapper::Binrw(args) = StructOpt::from_args();
    main(args)
}

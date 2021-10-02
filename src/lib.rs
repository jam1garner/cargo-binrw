#[cfg(feature = "cli")]
use structopt::StructOpt;

#[cfg_attr(feature = "cli", derive(StructOpt))]
pub struct Args {
    // TODO
}

pub fn main(args: Args) {
    todo!()
}

#[cfg(feature = "cli")]
pub fn main_from_args() {
    main(Args::from_args())
}

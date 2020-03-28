#[macro_use]
mod utils;
mod commands;

use commands::{Exec, init};
use structopt::StructOpt;

macro_rules! gen_subcmd_e {
    ($name:ident; $($cmd:ident($ty:ty)),*) => {
        #[derive(Debug, StructOpt)]
        enum $name {
            $(
                $cmd($ty),
            )*
        }

        impl Exec for $name {
            fn exec(&self) {
                match &self {
                    $($name::$cmd(subcmd) => subcmd.exec(),)*
                }
            }
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "enzo", about = "<something cool>")]
struct Opt {
    #[structopt(subcommand)]
    subcmd: SubCmd,
}

gen_subcmd_e!(SubCmd; Init(init::Opt));

fn main() {
    let opts = Opt::from_args();
    opts.subcmd.exec();
}


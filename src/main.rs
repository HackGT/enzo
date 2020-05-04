mod commands;
mod config;

use commands::*;
use config::context::Context;
use dirs;
use structopt::StructOpt;

// macro that creates the SubCmd enum and implements the Exec trait
macro_rules! gen_subcmd_e {
    ($name:ident; $($cmd:ident($ty:ty)),*) => {
        #[derive(Debug, StructOpt)]
        enum $name {
            $(
                $cmd($ty),
            )*
        }

        impl Exec for $name {
            fn exec(&self, context: &Context) {
                match &self {
                    $($name::$cmd(subcmd) => subcmd.exec(context),)*
                }
            }
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "enzo", about = "<something cool>")]
struct Opt {
    #[structopt(subcommand)]
    subcmd: Option<SubCmd>,
}

gen_subcmd_e!(SubCmd; Init(init::Opt), New(new::Opt));

fn main() {
    let mut config_path = match dirs::home_dir() {
        Some(path) => path,
        None => {
            error!("[FATAL] Could not determine the home directory!!!");
        }
    };

    if let Ok(context) = Context::load_from(&mut config_path) {
        let opts = Opt::from_args();
        if let Some(opt) = opts.subcmd {
            opt.exec(&context);
        } else {
            println!("Run enzo --help to view commands");
        }
    }
}

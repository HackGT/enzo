mod config;
mod utils;
mod workspace;

// use clap::App;
use std::process;

fn main() {
    let config_file_name = ".enzo.config.yaml";
    let conf = match config::read_config(config_file_name) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!("{:#?}", conf);
}

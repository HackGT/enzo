use clap::{App, Arg};
use std::process;

fn main() {
    let config_file_name = ".enzo.config.yaml";
    let _conf = match enzo::config::read_config(config_file_name) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // println!("{:#?}", conf);

    let matches = App::new("enzo")
        .version("0.0.1")
        .about("Workspace and repo management made fun ;)")
        .subcommand(
            App::new("clone")
                .about("Clone a git repo into a workspace")
                .arg(Arg::with_name("source").required(true))
                .arg(Arg::with_name("destination").required(true))
                .arg(
                    Arg::with_name("path")
                        .help("do not resolve any workspace name to a path")
                        .short("p")
                        .long("path"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("clone", Some(clone_matches)) => {
            println!(
                "Cloning {} into {}",
                clone_matches.value_of("source").unwrap(),
                clone_matches.value_of("destination").unwrap()
            );
            enzo::git::clone(String::new(), String::new());
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    };
}

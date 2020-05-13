use clap::{App, Arg};
use std::process;

// TODO write a try! macro to try something. if errs, print the error

fn main() {
    let config_file_name = ".enzo.config.yaml";
    let mut conf = match enzo::config::read_config(config_file_name) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let matches = App::new("enzo")
        .version("0.0.1")
        .about("Workspace and repo management made fun ;)")
        .subcommand(
            App::new("clone")
                .about("Clone a git repo into a workspace")
                .arg(Arg::with_name("src").required(true))
                .arg(Arg::with_name("dst").required(true))
                .arg(
                    Arg::with_name("path")
                        .help("do not resolve any workspace name to a path")
                        .short("p")
                        .long("path"),
                )
                .arg(
                    Arg::with_name("name")
                        .help("name of the new repo")
                        .short("n")
                        .long("name")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("new")
                .about("Clone a template repo into a workspace")
                .arg(Arg::with_name("src").required(true))
                .arg(Arg::with_name("dst").required(true))
                .arg(
                    Arg::with_name("path")
                        .help("do not resolve any workspace name to a path")
                        .short("p")
                        .long("path"),
                )
                .arg(
                    Arg::with_name("name")
                        .help("name of the new repo")
                        .short("n")
                        .long("name")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let res = match matches.subcommand() {
        ("clone", Some(clone_matches)) => enzo::clone(&mut conf, clone_matches),
        ("new", Some(new_matches)) => enzo::new(&mut conf, new_matches),
        ("", None) => Ok(()),
        _ => unreachable!(),
    };

    if let Err(e) = res {
        eprintln!("{}", e);
    }
}

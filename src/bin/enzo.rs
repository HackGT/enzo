use clap::{App, Arg};
use enzo::{config, utils};
use std::convert::TryFrom;
use std::process;

fn main() {
    let mut path = utils::get_home_dir().unwrap();
    path.push(".enzo.config.yaml");
    let mut conf = match config::Config::try_from(path.as_path()) {
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

    conf.write().unwrap();
}

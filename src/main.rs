use clap::{App, Arg};
use enzo::{
    config::global::Config,
    utils,
    utils::error::{EnzoError, EnzoErrorKind},
    workspace,
};
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), EnzoError> {
    // read the config file
    //
    // if it doesn't exist, use the methods from the library to create one
    //
    let mut config = read_config()?;

    let matches = App::new("enzo")
        .version("0.0.1")
        .about("Workspace and repo management made fun ;)")
        .subcommand(
            App::new("add")
                .about("add a `workspace` or a `task`")
                .arg(Arg::with_name("entity").required(true)),
        )
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
        .subcommand(
            App::new("todos")
                .about("Manage your todos")
                .arg(Arg::with_name("src")),
        )
        .get_matches();

    let res = match matches.subcommand() {
        ("add", Some(add_matches)) => match add_matches.value_of("entity") {
            Some("workspace") => {
                let (name, data) = workspace::query_workspace()?;
                config.add_workspace(name, data.path);
                Ok(())
            }
            Some("todo") => unimplemented!(),
            Some(not_supported) => {
                let msg = format!("Expected `workspace` or `todo`. Found: {}", not_supported);
                Err(EnzoError::new(msg, EnzoErrorKind::FatalError))
            }
            None => Err(EnzoError::new(
                "Expected `workspace` or `todo`. Found nothing.",
                EnzoErrorKind::FatalError,
            )),
        },
        ("clone", Some(clone_matches)) => {
            let src = clone_matches.value_of("src").unwrap();
            let dst = clone_matches.value_of("dst").unwrap();
            enzo::clone(&mut config, src, dst)
        }
        ("todos", todos_matches) => {
            if let Some(matches) = todos_matches {
                enzo::start_task_manager(&mut config, matches.value_of("src"))
            } else {
                enzo::start_task_manager(&mut config, None)
            }
        }
        _ => unreachable!(),
    };
    if let Err(e) = res {
        eprintln!("{}", e);
    }

    write_config(config)?;
    Ok(())
}

fn read_config() -> Result<Config, EnzoError> {
    let mut path = utils::get_home_dir()?;
    path.push(".enzo.config.yaml");
    if !path.exists() {
        // warn that the config file didn't exist
        let mut file = File::create(&path)?;
        file.write_all(Config::default().to_string()?.as_bytes())?;
    }
    Config::try_from(path)
}

fn write_config(config: Config) -> Result<(), EnzoError> {
    let mut path = utils::get_home_dir()?;
    path.push(".enzo.config.yaml");
    let mut file = File::create(&path)?;
    file.write_all(config.to_string()?.as_bytes())?;
    Ok(())
}

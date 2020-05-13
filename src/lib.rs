pub mod config;
pub mod git;
pub mod utils;
mod workspace;

use clap::ArgMatches;
use read_input::prelude::*;
use std::path::PathBuf;
use utils::error::{EnzoError, EnzoErrorKind};
use utils::query::Question;

fn resolve_src(src: &str) -> String {
    // TODO more robust src resolution; only once git supports ssh credentials and stuff
    format!("{}/{}.git", "https://github.com", src)
}

fn resolve_dst(config: &mut config::Config, dst: &str, name: &str) -> Result<PathBuf, EnzoError> {
    let res = config.resolve_path(dst.to_string());
    let mut dst = match res {
        Some(base_path) => base_path,
        None => {
            let (name, data) = workspace::query_workspace()?;
            let path = data.path.clone();
            config.add(name.0, data);
            path
        }
    };

    dst.push(name);

    Ok(dst)
}

fn read_name_from_stdin() -> Result<String, EnzoError> {
    let name = input::<String>()
        .msg(format!("{}", Question::new_question("Name of the repo")))
        .get();
    Ok(name)
}

fn name_helper<'a>(
    src: &str,
    user_provided_name: Option<&str>,
    read_from_stdin: bool,
) -> Result<String, EnzoError> {
    let name = match user_provided_name {
        Some(name) => name.to_string(),
        None => {
            if read_from_stdin {
                read_name_from_stdin()?
            } else {
                match get_repo_name(&src) {
                    Some(name) => name.to_string(),
                    None => return Err(EnzoError::new(format!(
                        "Failed to parse name of the repo from '{}'. It should be of the format 'username/repo_name'", src),
                        EnzoErrorKind::GitError,
                    )),
                }
            }
        }
    };

    Ok(name)
}

pub fn new(config: &mut config::Config, input: &ArgMatches) -> Result<(), EnzoError> {
    let src = resolve_src(input.value_of("src").unwrap());
    let name = name_helper(input.value_of("src").unwrap(), input.value_of("name"), true)?;
    let dst = resolve_dst(config, input.value_of("dst").unwrap(), name.as_str())?;

    git::clone(src, &dst)?;
    // remove the .git dir
    // git init

    Ok(())
}

pub fn clone(config: &mut config::Config, input: &ArgMatches) -> Result<(), EnzoError> {
    let src = resolve_src(input.value_of("src").unwrap());
    let name = name_helper(
        input.value_of("src").unwrap(),
        input.value_of("name"),
        false,
    )?;
    let dst = resolve_dst(config, input.value_of("dst").unwrap(), name.as_str())?;

    git::clone(src, &dst)
}

fn get_repo_name<'a>(src: &'a str) -> Option<&'a str> {
    match src.rfind("/") {
        Some(i) => Some(&src[i + 1..]),
        None => None,
    }
}

pub mod config;
pub mod git;
pub mod utils;
mod workspace;

use clap::ArgMatches;
use std::path::PathBuf;
use utils::error::{EnzoError, EnzoErrorType};

fn resolve_src(src: &str) -> String {
    // TODO more robust src resolution; only once git supports ssh credentials and stuff
    format!("{}/{}.git", "https://github.com", src)
}

fn resolve_dst(config: &mut config::Config, dst: &str, name: &str) -> Result<PathBuf, EnzoError> {
    let res = config.resolve_path(dst.to_string());
    let mut dst = match res {
        Some(base_path) => base_path,
        None => {
            let (name, data) = workspace::read_from_stdin()?;
            config.add(&name, &data);
            data.path.clone()
        }
    };

    dst.push(name);

    Ok(dst)
}

fn name_helper<'a>(
    src: &'a str,
    user_provided_name: Option<&'a str>,
) -> Result<&'a str, EnzoError> {
    let name = match user_provided_name {
        Some(name) => name,
        None => match get_repo_name(&src) {
            Some(name) => name,
            None => return Err(EnzoError::new(
                "Failed to parse the repo name. It should be of the format 'username/repo_name'",
                EnzoErrorType::GitError,
                None,
            )),
        },
    };
    Ok(name)
}

pub fn new(config: &mut config::Config, input: &ArgMatches) -> Result<(), EnzoError> {
    // 1. resolve src
    let src = resolve_src(input.value_of("src").unwrap());
    let name = name_helper(&src, input.value_of("name"))?;
    let dst = resolve_dst(config, input.value_of("dst").unwrap(), name)?;
    println!("src:{}\ndst:{:?}", src, dst);

    // clone_helper(config, src, dst, template_name)?;
    // 2. clone the repo

    // 3. delete the .git dir

    // 4. run git init
    Ok(())
}

pub fn clone(config: &mut config::Config, input: &ArgMatches) -> Result<(), EnzoError> {
    // required args
    let src = input.value_of("src").unwrap();
    let dst = input.value_of("dst").unwrap();

    // optional args
    let repo_name = if let Some(repo_name) = input.value_of("name") {
        repo_name
    } else {
        match get_repo_name(&src) {
            Some(name) => name,
            None => return Err(EnzoError::new(
                "Failed to parse the repo name. It should be of the format 'username/repo_name'",
                EnzoErrorType::GitError,
                None,
            )),
        }
    };

    let src = resolve_src(src);
    // 2. resolve the dst
    let dst = if let Some(mut dst) = config.resolve_path(dst.to_string()) {
        dst.push(repo_name.to_string());
        dst
    } else {
        // prompt user to create a workspace
        if let Ok((ref name, ref data)) = workspace::read_from_stdin() {
            config.add(name, data);
            data.path.clone()
        } else {
            return Err(EnzoError::new(
                "Could not read workspace from stdin",
                EnzoErrorType::FatalError,
                None,
            ));
        }
    };

    println!("source: {}", src);
    println!("dest: {:?}", dst);

    git::clone(src, &dst)
}

fn get_repo_name<'a>(src: &'a str) -> Option<&'a str> {
    match src.rfind("/") {
        Some(i) => Some(&src[i + 1..]),
        None => None,
    }
}

pub mod config;
pub mod git;
pub mod utils;
mod workspace;

use utils::error::{EnzoError, EnzoErrorType};

pub fn new(config: &mut config::Config, src: String, dst: String) -> Result<(), EnzoError> {
    // 1. read name of the repo from the user

    // 2. clone the repo
    // 3. delete the .git dir

    // 4. run git init
    Ok(())
}

pub fn clone(
    config: &mut config::Config,
    src: String,
    dst: String,
    repo_name: Option<&str>,
) -> Result<(), EnzoError> {
    // 1. resolve the src
    // TODO better handling of src
    //      detect if it is a url
    //      autodetect host and protocol
    //      use apt steps based on that
    let repo_name = if let Some(repo_name) = repo_name {
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

    let src = format!("{}/{}.git", "https://github.com", src);
    // 2. resolve the dst
    let dst = if let Some(mut dst) = config.resolve_path(dst) {
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

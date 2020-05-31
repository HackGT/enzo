// this is just a library
// It shouldn't be dependent on clap or any binary related dependency
//
// it should merely expose an API that the binary can use to wield its magic

pub mod config;
mod git;
mod project_config;
pub mod utils;
pub mod workspace;

use config::Config;
use std::path::PathBuf;
use utils::error::{EnzoError, EnzoErrorKind};
use workspace::WorkspaceName;

pub fn resolve_src(src: &str) -> Result<String, EnzoError> {
    let src = format!("{}/{}.git", "https://github.com", src);
    utils::info(format!("src = {}", src));
    Ok(src)
}

pub fn resolve_dst(config: &mut Config, dst: &str) -> Result<(WorkspaceName, PathBuf), EnzoError> {
    // dst -> hackgt/websites
    if let Some(tuple) = config.resolve_path(dst) {
        utils::info(format!("dst = {:?}", tuple.1));
        Ok(tuple)
    } else {
        let msg = format!("The name {} could not be resolved to a workspace.\nhint: Try creating a new workspace with `enzo add workspace`", dst);
        Err(EnzoError::new(msg, EnzoErrorKind::ConfigError))
    }
}

pub fn clone(config: &mut Config, src: &str, dst: &str) -> Result<(), EnzoError> {
    let repo_name = match get_repo_name(&src) {
        Some(name) => name,
        None => {
            let msg = format!(
                "Expected `src` to be of the form <username>/<repo_name>. Found: {}",
                src
            );
            return Err(EnzoError::new(msg, EnzoErrorKind::FatalError));
        }
    };
    let src = resolve_src(src)?;
    let (workspace_name, mut dst) = resolve_dst(config, dst)?;
    dst.push(repo_name);

    git::clone(src, &dst)?;
    config.add_project(workspace_name, dst)?;

    Ok(())
}

// fn resolve_src(src: &str) -> String {
//     // TODO more robust src resolution
//     format!("{}/{}.git", "https://github.com", src)
// }
//
// fn resolve_dst(config: &mut config::Config, dst: &str, name: &str) -> Result<PathBuf, EnzoError> {
//     let res = config.resolve_path(dst.to_string());
//     let mut dst = match res {
//         Some(base_path) => base_path,
//         None => {
//             let (name, data) = workspace::query_workspace()?;
//             let path = data.path.clone();
//             config.add(name.0, data);
//             path
//         }
//     };
//
//     dst.push(name);
//
//     Ok(dst)
// }
//
// fn read_name_from_stdin() -> Result<String, EnzoError> {
//     let name = input::<String>()
//         .msg(format!("{}", Question::new_question("Name of the repo")))
//         .get();
//     Ok(name)
// }
//
// fn name_helper<'a>(
//     src: &str,
//     user_provided_name: Option<&str>,
//     read_from_stdin: bool,
// ) -> Result<String, EnzoError> {
//     let name = match user_provided_name {
//         Some(name) => name.to_string(),
//         None => {
//             if read_from_stdin {
//                 read_name_from_stdin()?
//             } else {
//                 match get_repo_name(&src) {
//                     Some(name) => name.to_string(),
//                     None => return Err(EnzoError::new(format!(
//                         "Failed to parse name of the repo from '{}'. It should be of the format 'username/repo_name'", src),
//                         EnzoErrorKind::GitError,
//                     )),
//                 }
//             }
//         }
//     };
//
//     Ok(name)
// }

// pub fn new(config: &mut config::Config, input: &ArgMatches) -> Result<(), EnzoError> {
//     let src = resolve_src(input.value_of("src").unwrap());
//
//     let name = name_helper(input.value_of("src").unwrap(), input.value_of("name"), true)?;
//     let mut dst = resolve_dst(config, input.value_of("dst").unwrap(), name.as_str())?;
//
//     utils::info(format!("src = {}", src).as_str());
//     utils::info(format!("dst = {:?}", dst).as_str());
//
//     git::clone(src, &dst)?;
//
//     utils::info("removing the .git directory");
//
//     dst.push(".git");
//     fs::remove_dir_all(&dst)?;
//     dst.pop();
//
//     utils::info("running git init");
//
//     git2::Repository::init(dst).unwrap();
//
//     utils::success("git repo initialized");
//
//     // read the repo config file
//     // transfer todos from project config to repo config
//     // done!
//     Ok(())
// }
//
// pub fn clone(config: &mut config::Config, input: &ArgMatches) -> Result<(), EnzoError> {
//     let src = resolve_src(input.value_of("src").unwrap());
//
//     let name = name_helper(
//         input.value_of("src").unwrap(),
//         input.value_of("name"),
//         false,
//     )?;
//     let dst = resolve_dst(config, input.value_of("dst").unwrap(), name.as_str())?;
//
//     utils::info(format!("src = {}", src).as_str());
//     utils::info(format!("dst = {:?}", dst).as_str());
//
//     utils::info("initiating clone");
//     git::clone(src, &dst)?;
//     utils::success("cloned");
//     Ok(())
// }

fn get_repo_name<'a>(src: &'a str) -> Option<&'a str> {
    match src.rfind("/") {
        Some(i) => Some(&src[i + 1..]),
        None => None,
    }
}

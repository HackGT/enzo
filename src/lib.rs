pub mod config;
mod git;
mod todos;
pub mod utils;
pub mod workspace;

use config::global::Config;
use std::path::PathBuf;
use utils::error::{EnzoError, EnzoErrorKind};
use workspace::{project::Project, WorkspaceName};

pub fn resolve_src(src: &str) -> Result<String, EnzoError> {
    let src = format!("{}/{}.git", "https://github.com", src);
    utils::info(format!("src = {}", src));
    Ok(src)
}

pub fn resolve_dst(config: &mut Config, dst: &str) -> Result<(WorkspaceName, PathBuf), EnzoError> {
    if let Some(tuple) = config.resolve_path(dst) {
        Ok(tuple)
    } else {
        let msg = format!("The name {} could not be resolved to a workspace.\nhint: Try creating a new workspace with `enzo add workspace`", dst);
        // TODO fix the error kind
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
    utils::info(format!("dst = {:?}", dst));

    git::clone(&src, &dst)?;

    dst.push("enzo.yaml");
    let todos = todos::read_from(&dst)?;

    dst.pop();
    // TODO handle error more gracefully
    let name = dst.file_name().unwrap();
    let project = Project::new(name.to_str().unwrap().into(), workspace_name, src, todos);

    config.add_project(dst, project);

    Ok(())
}

pub fn start_task_manager<'a>(config: &'a mut Config, src: Option<&str>) -> Result<(), EnzoError> {
    let path = if let Some(src) = src {
        let (_, dst) = resolve_dst(config, src)?;
        utils::info(format!("src = {:?}", dst));
        dst
    } else {
        std::env::current_dir()?
    };
    let project = config.get_project_mut(&path).unwrap();
    let todos = &mut project.todos;
    todos::start(todos)?;
    Ok(())
}

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

fn get_repo_name<'a>(src: &'a str) -> Option<&'a str> {
    match src.rfind("/") {
        Some(i) => Some(&src[i + 1..]),
        None => None,
    }
}

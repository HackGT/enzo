pub mod config;
mod git;
mod todos;
pub mod utils;
pub mod workspace;

use config::{global::Config, section::ExecutionContext};
use std::fs;
use std::path::PathBuf;
use utils::error::{EnzoError, EnzoErrorKind};
use utils::query::{AnswerKind, Question};
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

pub fn clone(
    config: &mut Config,
    src: &str,
    dst: &str,
    name: Option<String>,
) -> Result<(), EnzoError> {
    let repo_name = if let Some(name) = name {
        name
    } else {
        match get_repo_name(&src) {
            Some(name) => name.to_string(),
            None => {
                let msg = format!(
                    "Expected `src` to be of the form <username>/<repo_name>. Found: {}",
                    src
                );
                return Err(EnzoError::new(msg, EnzoErrorKind::FatalError));
            }
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

// TODO clean up
pub fn new(config: &mut Config, src: &str, dst: &str) -> Result<(), EnzoError> {
    let question = Question::new_question("What is the name of your project?");
    let mut answer_kind = AnswerKind::Single(String::new());
    question.ask(&mut answer_kind);
    let name = match answer_kind {
        AnswerKind::Single(s) => s.clone(),
        _ => unreachable!(),
    };
    clone(config, src, dst, Some(name.clone()))?;

    let (_, mut dst) = resolve_dst(config, dst)?;
    dst.push(name);
    dst.push(".git");
    fs::remove_dir_all(&dst)?;
    dst.pop();
    git::init(&dst)?;
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
    let project = match config.get_project_mut(&path) {
        Some(project) => project,
        None => {
            return Err(EnzoError::new(
                format!("The project at {:?} does not exist", path),
                EnzoErrorKind::IOError,
            ))
        }
    };
    let todos = &mut project.todos;
    todos::start(todos)?;
    Ok(())
}

pub fn configure(config: &mut Config, src: Option<&str>) -> Result<(), EnzoError> {
    let mut path = if let Some(src) = src {
        let (_, dst) = resolve_dst(config, src)?;
        utils::info(format!("src = {:?}", dst));
        dst
    } else {
        std::env::current_dir()?
    };

    // TODO get remote from project

    let ctx = &ExecutionContext {
        repo: std::env::current_dir()?,
        curr: path.clone(),
        remote: String::from("testing"),
    };

    path.push("enzo.yaml");
    let project_config = config::project::read_from(&path)?;
    project_config.configure(ctx)?;
    Ok(())
}

pub fn deploy(config: &mut Config, src: Option<&str>) -> Result<(), EnzoError> {
    // TODO look for deployment source in project config file
    configure(config, src)?;
    Ok(())
}

fn get_repo_name<'a>(src: &'a str) -> Option<&'a str> {
    match src.rfind("/") {
        Some(i) => Some(&src[i + 1..]),
        None => None,
    }
}

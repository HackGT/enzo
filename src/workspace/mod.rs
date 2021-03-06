pub mod project;

use crate::utils::{self, query::Question};
use crate::workspace::project::Project;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceData {
    pub path: PathBuf,
    pub projects: Vec<Project>,
}

impl WorkspaceData {
    pub fn new(path: PathBuf, projects: Vec<Project>) -> WorkspaceData {
        WorkspaceData { path, projects }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WorkspaceName(pub String);

impl From<String> for WorkspaceName {
    fn from(name: String) -> Self {
        WorkspaceName(name)
    }
}

impl From<&str> for WorkspaceName {
    fn from(name: &str) -> Self {
        WorkspaceName(name.to_string())
    }
}

pub fn query_workspace() -> Result<(WorkspaceName, WorkspaceData), utils::error::EnzoError> {
    let name = input::<String>()
        .msg(format!("{}", Question::new_question("Workspace name")))
        .get();

    println!();

    let home = utils::get_home_dir()?;
    let mut ext;
    let mut base;

    loop {
        base = home.clone();
        ext = input::<String>()
            .msg(format!(
                "{}",
                Question::new("Path to workspace", Some("$HOME"), Some("$HOME/"), None)
            ))
            .get();
        base.push(ext);
        if !base.exists() {
            println!("Hmmm. looks like that wasn't a valid path. Try again\n");
        } else {
            break;
        }
    }
    // println!("{:?}", base);
    println!();

    // TODO change in the future to scan the directory and populate projects vec
    let data = WorkspaceData::new(base, vec![]);

    Ok((WorkspaceName(name.to_string()), data))
}

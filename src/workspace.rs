use crate::utils::{fatal_error::FatalError, get_home_dir, query};
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceData {
    pub path: PathBuf,
    pub projects: Vec<String>,
}

impl WorkspaceData {
    fn new(path: PathBuf, projects: Vec<String>) -> WorkspaceData {
        WorkspaceData { path, projects }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WorkspaceName(pub String);

pub fn read_from_stdin() -> Result<(WorkspaceName, WorkspaceData), FatalError> {
    let name = input::<String>()
        .msg(query("Workspace name", None, None))
        .get();

    let home = get_home_dir()?;
    let mut ext;
    let mut base;

    loop {
        base = home.clone();
        ext = input::<String>()
            .msg(query("Path to workspace", Some("$HOME"), Some("$HOME/")))
            .get();
        base.push(ext);
        if !base.exists() {
            println!("Hmmm. looks like that wasn't a valid path. Try again\n");
        } else {
            break;
        }
    }
    println!("{:?}", base);
    // TODO change in the future to scan the directory and populate projects vec
    let data = WorkspaceData::new(base, vec![]);

    Ok((WorkspaceName(name), data))
}

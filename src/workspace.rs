use crate::utils::{get_home_dir, FatalError};
use ansi_term::Color;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceData {
    path: PathBuf,
    projects: Vec<String>,
}

impl WorkspaceData {
    fn new(path: PathBuf, projects: Vec<String>) -> WorkspaceData {
        WorkspaceData { path, projects }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WorkspaceName(String);

pub fn read_from_stdin() -> Result<(WorkspaceName, WorkspaceData), FatalError> {
    let name = input::<String>()
        .msg(query("Workspace name", None, None))
        .get();
    let mut path = get_home_dir()?;
    let mut ext;
    loop {
        ext = input::<String>()
            .msg(query("Path to workspace", Some("$HOME"), Some("$HOME/")))
            .get();
        path.push(ext);
        if !path.exists() {
            println!("Hmmm. looks like that wasn't a valid path. Try again\n");
            path.pop();
        } else {
            break;
        }
    }

    // initialize with no projects
    // can change in the future to scan the directory and populate the file
    let data = WorkspaceData::new(path, vec![]);

    Ok((WorkspaceName(name), data))
}

fn query(question: &str, default: Option<&str>, pre: Option<&str>) -> String {
    format!(
        "{} {} {}\n> {}",
        Color::Green.bold().paint("?"),
        Color::White.bold().paint(question),
        if let Some(default) = default {
            format!("(default {})", Color::White.dimmed().paint(default))
        } else {
            String::new()
        },
        if let Some(pre) = pre {
            format!("{}", Color::Yellow.paint(pre))
        } else {
            String::new()
        }
    )
}

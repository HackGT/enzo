use crate::utils::error::{EnzoError, EnzoErrorKind};
use crate::workspace::{self, WorkspaceData, WorkspaceName};
use ansi_term::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    path: PathBuf,
    workspaces: HashMap<WorkspaceName, WorkspaceData>,
}

impl TryFrom<&Path> for Config {
    type Error = EnzoError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut config = Config {
            path: path.to_path_buf(),
            workspaces: HashMap::new(),
        };
        if path.exists() {
            config.read()?;
        } else {
            let (name, data) = workspace::query_workspace()?;
            config.workspaces.insert(name, data);
            config.write()?;
        }
        Ok(config)
    }
}

impl Config {
    pub fn read(&mut self) -> Result<(), EnzoError> {
        if !self.path.exists() {
            return Err(EnzoError::new(
                format!("Configuration file does not exist in {:?}", self.path),
                EnzoErrorKind::ConfigError,
            ));
        }

        let mut file = File::open(&self.path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let Config { ref workspaces, .. } = serde_yaml::from_str(buffer.as_str())?;
        self.workspaces = workspaces.clone();
        Ok(())
    }

    pub fn write(&mut self) -> Result<(), EnzoError> {
        let mut file = File::create(&self.path)?;
        let s = serde_yaml::to_string(&self)?;
        file.write_all(s.as_bytes())?;
        Ok(())
    }

    pub fn add(&mut self, name: String, data: WorkspaceData) -> Option<WorkspaceData> {
        self.workspaces.insert(WorkspaceName(name), data)
    }

    pub fn remove(&mut self, name: String) -> Option<WorkspaceData> {
        self.workspaces.remove(&WorkspaceName(name))
    }

    pub fn get(&mut self, name: String) -> Option<&WorkspaceData> {
        self.workspaces.get(&WorkspaceName(name))
    }

    pub fn get_path(&mut self, name: String) -> Option<&PathBuf> {
        if let Some(data) = self.get(name) {
            Some(&data.path)
        } else {
            None
        }
    }

    pub fn resolve_path(&self, path: String) -> Option<PathBuf> {
        let mut i = path.len();
        if path.ends_with("/") {
            i = i - 1;
        }
        while i > 0 {
            if let Some(val) = self.get_workspace_data(&path[..i]) {
                let mut resolved_path = val.path.clone();
                if path[i..].len() > 0 {
                    resolved_path.push(&path[i + 1..]);
                }
                return Some(resolved_path);
            } else {
                i = path[..i].rfind("/").unwrap_or(0);
            }
        }
        None
    }

    fn get_workspace_data(&self, name: &str) -> Option<&WorkspaceData> {
        self.workspaces.get(&WorkspaceName(name.to_string()))
    }
}

fn print_warning(msg: String) {
    println!(
        "{} {}",
        Color::Yellow.bold().paint("configuration warning:"),
        msg
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolve_path_none() {
        let workspaces = HashMap::new();

        let config = Config {
            path: PathBuf::new(),
            workspaces,
        };

        assert_eq!(config.resolve_path("hackgt".to_string()), None);
        assert_eq!(config.resolve_path("".to_string()), None);
        assert_eq!(config.resolve_path("/".to_string()), None);
    }

    #[test]
    fn resolve_path_some() {
        let input = vec![
            ("hackgt", "dev/hackgt"),
            ("hackgt.websites", "dev/hackgt/websites/"),
            ("college", "life/teen/college"),
            ("college/hw", "work/college/more_work/hw/"),
        ];

        let mut workspaces = HashMap::new();
        for (key, value) in input {
            workspaces.insert(
                WorkspaceName(String::from(key)),
                WorkspaceData::new(PathBuf::from(value), vec![]),
            );
        }

        let config = Config {
            path: PathBuf::new(),
            workspaces,
        };

        assert_eq!(
            config.resolve_path("hackgt".to_string()),
            Some(PathBuf::from("dev/hackgt"))
        );
        assert_eq!(
            config.resolve_path("hackgt/".to_string()),
            Some(PathBuf::from("dev/hackgt"))
        );
        assert_eq!(
            config.resolve_path("hackgt.websites".to_string()),
            Some(PathBuf::from("dev/hackgt/websites"))
        );
        assert_eq!(
            config.resolve_path("hackgt.websites/horizons".to_string()),
            Some(PathBuf::from("dev/hackgt/websites/horizons"))
        );
        assert_eq!(
            config.resolve_path("college/sophomore/fall2018/".to_string()),
            Some(PathBuf::from("life/teen/college/sophomore/fall2018"))
        );
        assert_eq!(
            config.resolve_path("college/hw/cs2110/prj1".to_string()),
            Some(PathBuf::from("work/college/more_work/hw/cs2110/prj1"))
        );
    }
}

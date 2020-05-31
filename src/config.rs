use crate::utils::error::{EnzoError, EnzoErrorKind};
use crate::workspace::{WorkspaceData, WorkspaceName};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// TODO remove the path from this struct; it is not necessary
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    workspaces: HashMap<WorkspaceName, WorkspaceData>,
}

impl TryFrom<PathBuf> for Config {
    type Error = EnzoError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if path.exists() {
            let mut file = File::open(path)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            let config = serde_yaml::from_str(&buffer)?;
            Ok(config)
        } else {
            let msg = format!("Could not find `enzo.config.yaml` at {:?}", path);
            Err(EnzoError::new(msg, EnzoErrorKind::ConfigError))
        }
    }
}

impl Config {
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

    fn get_workspace_data(&self, name: &str) -> Option<&WorkspaceData> {
        self.workspaces.get(&WorkspaceName(name.to_string()))
    }

    pub fn resolve_path(&self, path: &str) -> Option<(WorkspaceName, PathBuf)> {
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
                return Some((WorkspaceName(path[..i].to_string()), resolved_path));
            } else {
                i = path[..i].rfind("/").unwrap_or(0);
            }
        }
        None
    }

    pub fn to_string(&self) -> Result<String, EnzoError> {
        let s = serde_yaml::to_string(&self)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolve_path_none() {
        let workspaces = HashMap::new();

        let config = Config { workspaces };

        assert_eq!(config.resolve_path("hackgt"), None);
        assert_eq!(config.resolve_path(""), None);
        assert_eq!(config.resolve_path("/"), None);
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

        let config = Config { workspaces };

        assert_eq!(
            config.resolve_path("hackgt"),
            Some(PathBuf::from("dev/hackgt"))
        );
        assert_eq!(
            config.resolve_path("hackgt/"),
            Some(PathBuf::from("dev/hackgt"))
        );
        assert_eq!(
            config.resolve_path("hackgt.websites"),
            Some(PathBuf::from("dev/hackgt/websites"))
        );
        assert_eq!(
            config.resolve_path("hackgt.websites/horizons"),
            Some(PathBuf::from("dev/hackgt/websites/horizons"))
        );
        assert_eq!(
            config.resolve_path("college/sophomore/fall2018/"),
            Some(PathBuf::from("life/teen/college/sophomore/fall2018"))
        );
        assert_eq!(
            config.resolve_path("college/hw/cs2110/prj1"),
            Some(PathBuf::from("work/college/more_work/hw/cs2110/prj1"))
        );
    }
}

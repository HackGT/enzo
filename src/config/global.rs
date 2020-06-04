use crate::{
    utils::error::{EnzoError, EnzoErrorKind},
    workspace::{project::Project, WorkspaceName},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, fs::File, io::prelude::*, path::PathBuf};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    workspaces: HashMap<WorkspaceName, PathBuf>,
    projects: HashMap<PathBuf, Project>,
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
    pub fn add_workspace<T: Into<WorkspaceName>>(
        &mut self,
        name: T,
        path: PathBuf,
    ) -> Option<PathBuf> {
        self.workspaces.insert(name.into(), path)
    }

    pub fn remove_workspace<'a, T>(&mut self, name: &'a T) -> Option<PathBuf>
    where
        &'a T: Into<&'a WorkspaceName>,
    {
        self.workspaces.remove(name.into())
    }

    pub fn get_path<'a, T>(&self, name: &'a T) -> Option<&PathBuf>
    where
        &'a T: Into<&'a WorkspaceName>,
    {
        self.workspaces.get(name.into())
    }

    pub fn get_path_mut<'a, T>(&mut self, name: &'a T) -> Option<&mut PathBuf>
    where
        &'a T: Into<&'a WorkspaceName>,
    {
        self.workspaces.get_mut(name.into())
    }

    pub fn add_project(&mut self, path: PathBuf, project: Project) -> Option<Project> {
        self.projects.insert(path, project)
    }

    pub fn remove_project(&mut self, path: &PathBuf) -> Option<Project> {
        self.projects.remove(path)
    }

    pub fn get_project(&self, path: &PathBuf) -> Option<&Project> {
        self.projects.get(path)
    }

    pub fn get_project_mut(&mut self, path: &PathBuf) -> Option<&mut Project> {
        self.projects.get_mut(path)
    }

    pub fn resolve_path<'a, T: Into<&'a str>>(&self, path: T) -> Option<(WorkspaceName, PathBuf)> {
        let path: &str = path.into();
        let mut i = path.len();
        if path.ends_with("/") {
            i = i - 1;
        }
        while i > 0 {
            if let Some(path_buf) = self.get_path(&path[..i].into()) {
                let mut resolved_path = path_buf.clone();
                if path[i..].len() > 0 {
                    resolved_path.push(&path[i + 1..]);
                }
                return Some((path[..i].into(), resolved_path));
            } else {
                i = path[..i].rfind("/").unwrap_or(0);
            }
        }
        None
    }

    pub fn to_string(&self) -> Result<String, EnzoError> {
        let s = serde_yaml::to_string(self)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolve_path_none() {
        let workspaces = HashMap::new();
        let projects = HashMap::new();

        let config = Config {
            workspaces,
            projects,
        };

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
        let projects = HashMap::new();

        for (key, value) in input {
            workspaces.insert(WorkspaceName(String::from(key)), PathBuf::from(value));
        }

        let config = Config {
            workspaces,
            projects,
        };

        assert_eq!(
            config.resolve_path("hackgt"),
            Some((WorkspaceName("hackgt".into()), PathBuf::from("dev/hackgt")))
        );
        assert_eq!(
            config.resolve_path("hackgt/"),
            Some((WorkspaceName("hackgt".into()), PathBuf::from("dev/hackgt")))
        );
        assert_eq!(
            config.resolve_path("hackgt.websites"),
            Some((
                WorkspaceName("hackgt.websites".into()),
                PathBuf::from("dev/hackgt/websites")
            ))
        );
        assert_eq!(
            config.resolve_path("hackgt.websites/horizons"),
            Some((
                WorkspaceName("hackgt.websites".into()),
                PathBuf::from("dev/hackgt/websites/horizons")
            ))
        );
        assert_eq!(
            config.resolve_path("college/sophomore/fall2018/"),
            Some((
                WorkspaceName("college".into()),
                PathBuf::from("life/teen/college/sophomore/fall2018")
            ))
        );
        assert_eq!(
            config.resolve_path("college/hw/cs2110/prj1"),
            Some((
                WorkspaceName("college/hw".into()),
                PathBuf::from("work/college/more_work/hw/cs2110/prj1")
            ))
        );
    }
}

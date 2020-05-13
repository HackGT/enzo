use crate::utils;
use crate::utils::error::{EnzoError, EnzoErrorType};
use crate::workspace::{self, WorkspaceData, WorkspaceName};
use ansi_term::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    recent_workspace: WorkspaceName,
    workspaces: HashMap<WorkspaceName, WorkspaceData>,
}

impl Config {
    pub fn new(
        recent_workspace: WorkspaceName,
        workspaces: HashMap<WorkspaceName, WorkspaceData>,
    ) -> Config {
        Config {
            recent_workspace,
            workspaces,
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

    pub fn add(&mut self, name: &WorkspaceName, data: &WorkspaceData) {
        self.workspaces.insert(name.clone(), data.clone());
    }

    pub fn get_path(&self, name: &WorkspaceName) -> Option<&PathBuf> {
        if let Some(data) = self.workspaces.get(name) {
            Some(&data.path)
        } else {
            None
        }
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

// TODO change from Box<dyn Error> to something that doesn't depend on dynamic dispatch
pub fn read_config(name: &str) -> Result<Config, EnzoError> {
    let mut config_file_path = utils::get_home_dir()?;
    config_file_path.push(name);

    if !config_file_path.exists() {
        print_warning(format!("Couldn't find {}", Color::Blue.bold().paint(name)));
        create_config_file(&config_file_path)?;
    }

    let mut file = match File::open(&config_file_path) {
        Ok(handle) => handle,
        Err(_) => {
            // critical error, because the config file should've been made by now
            return Err(EnzoError::new(
                "Config file could not be opened",
                EnzoErrorType::FatalError,
                None,
            ));
        }
    };

    let mut buffer = String::new();

    // TODO handle this error more gracefully
    if let Err(e) = file.read_to_string(&mut buffer) {
        return Err(EnzoError::new(
            "Failed to read from config file",
            EnzoErrorType::ConfigError,
            Some(format!("{:?}", e)),
        ));
    }

    let config = match serde_yaml::from_str(buffer.as_str()) {
        Ok(config) => config,
        Err(e) => {
            return Err(EnzoError::new(
                "Failed to deserialize config file.",
                EnzoErrorType::ConfigError,
                Some(format!("{:?}", e)),
            ))
        }
    };

    Ok(config)
}

fn create_config_file(path: &PathBuf) -> Result<File, EnzoError> {
    let mut file = match File::create(path) {
        Ok(handle) => handle,
        Err(_) => {
            return Err(EnzoError::new(
                format!("Couldn't create config file in {:?}", path).as_str(),
                EnzoErrorType::ConfigError,
                None,
            ));
        }
    };
    let (name, data) = workspace::read_from_stdin()?;
    let mut workspaces = HashMap::new();
    workspaces.insert(name.clone(), data);

    let config = Config::new(name, workspaces);

    let s = match serde_yaml::to_string(&config) {
        Ok(s) => s,
        Err(e) => {
            return Err(EnzoError::new(
                "Failed to serialize config data.",
                EnzoErrorType::ConfigError,
                Some(format!("{:?}", e)),
            ))
        }
    };

    if let Err(_) = file.write(s.as_bytes()) {
        return Err(EnzoError::new(
            "Could not write config info to the configuration file",
            EnzoErrorType::ConfigError,
            None,
        ));
    }

    Ok(file)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolve_path_none() {
        let recent_workspace = WorkspaceName(String::from(""));
        let workspaces = HashMap::new();

        let config = Config::new(recent_workspace, workspaces);

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
        let recent_workspace = WorkspaceName(String::from("hackgt"));

        let config = Config::new(recent_workspace, workspaces);

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

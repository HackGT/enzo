use crate::utils::{fatal_error::FatalError, get_home_dir};
use crate::workspace::{self, WorkspaceData, WorkspaceName};
use ansi_term::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    recent_workspace: WorkspaceName,
    pub workspaces: HashMap<WorkspaceName, WorkspaceData>,
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

    fn resolve_path(&self, path: String) -> Option<PathBuf> {
        let mut i = path.len();
        if path.rfind("/").unwrap_or(0) == path.len() {
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

#[derive(Debug)]
pub struct ConfigError {
    msg: String,
}

impl ConfigError {
    fn new(msg: &str) -> ConfigError {
        ConfigError {
            msg: msg.to_owned(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            Color::Red.bold().paint("configuration file error"),
            self.msg
        )
    }
}

fn print_warning(msg: String) {
    println!(
        "{} {}",
        Color::Yellow.bold().paint("configuration warning:"),
        msg
    );
}

impl Error for ConfigError {}

// TODO change from Box<dyn Error> to something that doesn't depend on dynamic dispatch
pub fn read_config(name: &str) -> Result<Config, Box<dyn Error>> {
    let mut config_file_path = get_home_dir()?;
    config_file_path.push(name);

    if !config_file_path.exists() {
        print_warning(format!("Couldn't find {}", Color::Blue.bold().paint(name)));
        create_config_file(&config_file_path)?;
    }

    let mut file = match File::open(&config_file_path) {
        Ok(handle) => handle,
        Err(_) => {
            // critical error, because the config file should've been made by now
            return Err(Box::new(FatalError::new(
                "Could not find .enzo.config.yaml file",
            )));
        }
    };

    let mut buffer = String::new();

    // TODO handle this error more gracefully
    file.read_to_string(&mut buffer)?;

    let config = serde_yaml::from_str(buffer.as_str())?;

    Ok(config)
}

fn create_config_file(path: &PathBuf) -> Result<File, Box<dyn Error>> {
    let mut file = match File::create(path) {
        Ok(handle) => handle,
        Err(_) => {
            return Err(Box::new(ConfigError::new(
                format!("Couldn't create config file in {:?}", path).as_str(),
            )))
        }
    };
    let (name, data) = workspace::read_from_stdin()?;
    let mut workspaces = HashMap::new();
    workspaces.insert(name.clone(), data);

    let config = Config::new(name, workspaces);

    // TODO handle this error more gracefully
    let s = serde_yaml::to_string(&config)?;

    if let Err(_) = file.write(s.as_bytes()) {
        return Err(Box::new(ConfigError::new(
            "Could not write config info to the configuration file",
        )));
    }

    Ok(file)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolve_path() {
        let input = vec![
            ("hackgt", "dev/hackgt"),
            ("hackgt/websites", "dev/hackgt/websites/"),
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
            config.resolve_path("hackgt/websites".to_string()),
            Some(PathBuf::from("dev/hackgt/websites"))
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

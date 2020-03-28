use crate::commands::Exec;
use crate::utils::config;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt { }

impl Exec for Opt {
    fn exec(&self) {
        if Path::new("./enzo.yaml").exists() {
            // add validation code
            warn!("enzo.yaml file already exists");
        } else {
            if let Ok(()) = create_config_file() {
                success!("Successfully created enzo.yaml");
            } else {
                error!("enzo.yaml file could not be created :(");
            }
        }
    }
}

fn create_config_file() -> std::io::Result<()> {
    info!("Creating an enzo.yaml for you");
    
    let mut file = File::create("enzo.yaml")?;
    file.write_all(config::default_config().as_bytes())?;
    Ok(())
}


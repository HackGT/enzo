use crate::commands::Exec;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt { 
    repo: String,
}

impl Exec for Opt {
    fn exec(&self) {
        info!("The repo name passed in is", self.repo);
    }
}

// next thing to set up is reading from a config file
// the config file contains
//      $WORKSPACE => the directory in which you want to clone your project
//      a list of aliases
//          alias est = event-site
//
//
// how are you going to orchestrate this?
//      check if a config file exists
//          if it doesnt, 
//              then go through setting it up
//          else
//              load the config file
//              set it up in a global data structure
//
//              try to use the data from there!
//          

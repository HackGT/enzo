mod config;
mod utils;
mod workspace;

use clap::{App, Arg};
use std::io::Write;
use std::path::Path;
use std::process::{self, Command, Stdio};

fn main() {
    let config_file_name = ".enzo.config.yaml";
    let _conf = match config::read_config(config_file_name) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // println!("{:#?}", conf);

    let matches = App::new("enzo")
        .version("0.0.1")
        .about("Workspace and repo management made fun ;)")
        .subcommand(
            App::new("clone")
                .about("Clone a git repo into a workspace")
                .arg(Arg::with_name("source").required(true))
                .arg(Arg::with_name("destination").required(true))
                .arg(
                    Arg::with_name("path")
                        .help("do not resolve any workspace name to a path")
                        .short("p")
                        .long("path"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("clone", Some(clone_matches)) => {
            println!(
                "Cloning {} into {}",
                clone_matches.value_of("source").unwrap(),
                clone_matches.value_of("destination").unwrap()
            );
            clone(String::new(), String::new());
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    };
}

fn clone(_src: String, _dst: String) {
    if let Some((username, password)) = get_git_credentials("https", "github.com") {
        // let mut callbacks = git2::RemoteCallbacks::new();
        // callbacks.credentials(|_, _, _| git2::Cred::userpass_plaintext(&username, &password));

        // let mut fo = git2::FetchOptions::new();
        // fo.remote_callbacks(callbacks);

        // let mut builder = git2::build::RepoBuilder::new();
        // builder.fetch_options(fo);

        // builder
        //     .clone("https://github.com/HackGT/enzo.git", Path::new("est"))
        //     .unwrap();
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            git2::Cred::userpass_plaintext(&username, &password)
        });

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        builder
            .clone("https://github.com/HackGT/est.git", Path::new("est"))
            .unwrap();
    } else {
    };
}

// TODO better error handling and messages
// TODO more readable code and variable names
fn get_git_credentials(protocol: &str, host: &str) -> Option<(String, String)> {
    let input_str = format!("protocol={}\nhost={}\n\n", protocol, host);
    let mut child = Command::new("git")
        .args(&["credential", "fill"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run git command");
    {
        let stdin = child.stdin.as_mut().expect("failed to open it");
        stdin.write_all(input_str.as_bytes()).expect("fail guys");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    let s = String::from_utf8_lossy(&output.stdout);

    let split_index = input_str.len() - 1;

    let (_, v) = s.split_at(split_index);
    let values = v
        .lines()
        .zip(&mut ["username=", "password="].iter())
        .map(|(line, key)| {
            let (_, val) = line.split_at(key.len());
            val
        })
        .collect::<Vec<&str>>();

    if let [username, password] = values[0..2] {
        Some((username.to_string(), password.to_string()))
    } else {
        None
    }
}

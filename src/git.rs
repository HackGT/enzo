use crate::utils::error::{EnzoError, EnzoErrorType};
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

// TODO add fallbacks if credential fetching fails
pub fn clone(src: String, dst: &Path) -> Result<(), EnzoError> {
    if let Some((username, password)) = get_git_credentials("https", "github.com") {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::userpass_plaintext(&username, &password)
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        if let Err(e) = builder.clone(src.as_str(), dst) {
            Err(EnzoError::new(
                format!(
                    "Failed to clone {} into {}\ncause: {:?}",
                    src,
                    dst.to_string_lossy(),
                    e
                )
                .as_str(),
                EnzoErrorType::GitError,
            ))
        } else {
            Ok(())
        }
    } else {
        Err(EnzoError::new(
            "Failed to obtain git credentials for protocol=https host=github.com",
            EnzoErrorType::GitError,
        ))
    }
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

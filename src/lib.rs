pub mod config;
pub mod git;
mod utils;
mod workspace;

use utils::fatal_error::FatalError;

pub fn clone(config: &mut config::Config, src: String, dst: String) -> Result<(), FatalError> {
    // 1. resolve the src
    // TODO better handling of src
    //      detect if it is a url
    //      autodetect host and protocol
    //      use apt steps based on that
    let src = format!("{}/{}.git", "https://github.com", src);

    // 2. resolve the dst
    let dst = if let Some(dst) = config.resolve_path(dst) {
        dst
    } else {
        // prompt user to create a workspace
        if let Ok((ref name, ref data)) = workspace::read_from_stdin() {
            config.add(name, data);
            data.path.clone()
        } else {
            return Err(FatalError::new("Could not add workspace to the config"));
        }
    };

    println!("source: {}", src);
    println!("dest: {:?}", dst);

    // git::clone(src, &dst);
    Ok(())
}

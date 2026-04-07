use crate::error::Error;

/// Changes the shell's working directory.
///
/// `args[0]` is the target path. Falls back to `$HOME` when `args` is empty.
///
/// # Errors
/// Returns [`Error::Os`] when the OS rejects the directory change.
pub fn run(args: &[String]) -> Result<(), Error> {
    let target = match args.first() {
        Some(p) => p.clone(),
        None    => std::env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    };

    std::env::set_current_dir(&target)
        .map_err(|e| Error::Os(format!("cd: {target}: {e}")))
}
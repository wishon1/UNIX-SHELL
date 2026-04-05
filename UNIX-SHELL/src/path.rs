use std::path::Path;

/// Resolve the the command by searching for the full executable path in "PATH"
///
/// for example the user types ls this dir looks for /usr/local/bin/ls
/// in the "PATH" and resolves it
///
/// Constructed once when the [`crate::shell::Shell`] starts so that the
/// environment variable is read exactly once, not on every command.

pub struct PathResolver {
    // directory to search, in order. Empty when `PATH` is unset.
    dirs: Vec<String>,
}

impl PathResolver {
    // Reads `PATH` from the enviromant and builds the search list
    pub fn new() -> Self {
        let dirs = std::env::var("PATH")
            .unwrap_or_default()
            .split(':')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();

        Self { dirs }      
    }
}
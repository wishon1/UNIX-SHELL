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

    /// Returns the full path to an executable, or `None` if not found.
    /// if `name` contains a `/` its treated as a literal path.
    /// otherwisw every directory in `PATH` is probbed as `dir/name`
    pub fn resolve(&self, name: &str) -> Option<String> {
        if name.contains('/') {
            return Self::is_executable(name).then(|| name.to_string());
        }

        self.dirs
            .ietr()
            .map(|dir| format!("{dir}/{name}"))
    }

    /// Returns `true` when `path` exits and has at least one executable bit.
    fn is_executable(path: &str) -> bool {
        use std::os::unix::fs::permissionExt;
        path::new(path)
            .metadata()
            .map(|m| m.permission().mode())
            .unwrap_or_(false)
    }
}
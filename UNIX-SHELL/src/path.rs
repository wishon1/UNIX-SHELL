use std::path::Path;

/// Resolves a command name to a full executable path by searching `PATH`.
///
/// Constructed once when the [`crate::shell::Shell`] starts so the
/// environment variable is read exactly once, not on every command.
pub struct PathResolver {
    /// Directories to search, in order. Empty when `PATH` is unset.
    dirs: Vec<String>,
}

impl PathResolver {
    /// Reads `PATH` from the environment and builds the search list.
    pub fn new() -> Self {
        let path_var = std::env::var("PATH").unwrap_or_default();
        let split_path_var = path_var.split(':');

        let mut dirs = Vec::new();
        for part in split_path_var {
            if !part.is_empty() {
                dirs.push(part.to_string());
            }
        }
        Self { dirs }
    }

    /// Returns the full path to an executable, or `None` if not found.
    ///
    /// If `name` contains a `/` it is treated as a literal path.
    /// Otherwise every directory in `PATH` is probed as `dir/name`.
    pub fn resolve(&self, name: &str) -> Option<String> {
        if name.contains('/') {
            if Self::is_executable(name) {
                return Some(name.to_string());
            } else {
                return None;
            }
        }

        for dir in &self.dirs {
            let full_path = format!("{}/{}", dir, name);
            if Self::is_executable(&full_path) {
                return Some(full_path);
            }
        }
        None
    }

    /// Returns `true` when `path` exists and has at least one executable bit.
    fn is_executable(path: &str) -> bool {
        use std::os::unix::fs::PermissionsExt;
        Path::new(path)
            .metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
}
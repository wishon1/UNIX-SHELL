use std::fmt;

/// All errors the shell can produce during a single REPL iteration.
///
/// Implements [`std::error::Error`] so callers can use the standard
/// error-handling ecosystem — trait objects, `?` chaining, etc.
#[derive(Debug)]
pub enum Error {
    /// The input line was blank or contained only whitespace.
    EmptyInput,

    /// No executable matching `name` was found on `PATH`
    CommandNotFound(String),

    /// An OS-level call failed (spawn, chdir, etc.).
    Os(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyInput => Ok(()),
            Error::CommandNotFound(name) => write!(f, "rush: {name}: command not found"),
            Error::Os(msg) => write!(f, "rush: {msg}"),
        }
    }
}

/// Implementing [`std::error::Error`] makes `Error` a first-class citizen
/// of Rust's error-handling ecosystem. Required for any production crate.
impl std::error::Error for Error {}
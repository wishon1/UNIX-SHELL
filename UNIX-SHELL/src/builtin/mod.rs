mod cd;
mod env;
mod exit;

use crate::error::Error;

/// Every built-in command the shell handles internally.
///
/// Built-ins run inside the shell process — they cannot be forked into a
/// child because they affect shell-owned state (working directory, process
/// exit code, environment).
///
/// Represented as an enum so dispatch is a single match with zero heap
/// allocation, as opposed to `Box<dyn Trait>`

pub enum Builtin {
    Cd,
    Env,
    Exit,
}

impl Builtin {
    /// Returns the [`Builtin`] matching `name`, or `None` if it is not one.
    ///
    /// `None` signals the caller to fall through to external execution.
    pub fn lookup(name: &str) -> Option<Self> {
        match name {
            "cd" => Some(Self::Cd),
            "env" => Some(Self::Env),
            "exit" => Some(Self::Exit),
            _      => None
        }
    }

    /// Executes this built-in with the provided arguments.
    ///
    /// `args` does not include the command name — only what follows it.
    ///
    /// # Errors
    /// Returns [`Error::Os`] when an underlying OS call fails.
    pub fn run(&self, args: &[String]) -> Result<(), Error> {
        match self {
            Self::Cd => cd::run(args),
            Self::Env => env::run(args),
            Self::Exit => exit::run(args),
        }
    }
}
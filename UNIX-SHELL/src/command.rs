use crate::path::PathResolver;
use crate::error::Error;
use crate::builtin::Builtin;

/// A parsed shell command ready for execution.
///
/// `name` and `args` are kept as separate fields. Flattening them into a
/// single `Vec` where index 0 is the name (the C `argv` layout) leaks
/// indexing arithmetic into every call site that touches this type.
#[derive(Debug, PartialEq)]
pub struct Command {
    /// The program name or built-in, exactly as typed.
    pub name: String,
    /// Arguments to the program, not including the name itself.
    pub args: Vec<String>,
}

impl Command {
    /// Parses a raw input line into a [`Command`].
    ///
    /// Named `parse` rather than `new` because it is a fallible
    /// transformation of input data, not a trivial construction.
    ///
    /// # Errors
    /// Returns [`Error::EmptyInput`] when `line` is blank or whitespace-only.
    pub fn parse(line: &str) -> Result<Self, Error> {
        let mut tokens = line.split_whitespace();

        // Try to get the first word.
        let name = match tokens.next() {
            Some(word) => word.to_string(),
            None       => return Err(Error::EmptyInput),
        };

        // Collect remaining tokens as arguments.
        let mut args = Vec::new();
        for token in tokens {
            args.push(token.to_string());
        }

        Ok(Self { name, args })
    }

    /// Dispatches this command: built-ins run in-process, everything else
    /// is resolved via `PATH` and spawned as a child process.
    ///
    /// `resolver` is passed in so the caller (the REPL) constructs it once
    /// and reuses it across commands, rather than re-reading `PATH` each time.
    ///
    /// # Errors
    /// Propagates [`Error::CommandNotFound`] or [`Error::Os`].
    pub fn run(&self, resolver: &PathResolver) -> Result<(), Error> {
        if let Some(builtin) = Builtin::lookup(&self.name) {
            return builtin.run(&self.args);
        }

        self.execute(resolver)
    }

    /// Spawns this command as an external child process and waits for it.
    fn execute(&self, resolver: &PathResolver) -> Result<(), Error> {
        let path = resolver
            .resolve(&self.name)
            .ok_or_else(|| Error::CommandNotFound(self.name.clone()))?;

        std::process::Command::new(&path)
            .args(&self.args)
            .status()
            .map_err(|e| Error::Os(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_name_and_args() {
        let cmd = Command::parse("echo hello world").unwrap();
        assert_eq!(cmd.name, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn single_token_yields_empty_args() {
        let cmd = Command::parse("ls").unwrap();
        assert_eq!(cmd.name, "ls");
        assert!(cmd.args.is_empty());
    }
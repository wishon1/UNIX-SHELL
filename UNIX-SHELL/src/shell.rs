use std::io::{self, BufRead, Write};

use crate::command::Command;
use crate::error::Error;
use crate::path::PathResolver;

/// The shell's read-eval-print loop and all state it needs to run it.
///
/// Constructed once in `main`. Owns the [`PathResolver`] so `PATH` is
/// read from the environment exactly once for the lifetime of the shell.
pub struct Shell {
    /// Pre-built resolver reused for every external command lookup.
    resolver: PathResolver,
    /// `true` when stdout is an interactive terminal; suppresses prompt otherwise.
    interactive: bool,
}

impl Shell {
    /// Constructs a [`Shell`], reading `PATH` and detecting the terminal once.
    pub fn new() -> Self {
        Self {
            resolver:    PathResolver::new(),
            interactive: io::IsTerminal::is_terminal(&io::stdout()),
        }
    }

    /// Runs the REPL until EOF, returning the process exit code to `main`.
    ///
    /// Returning an exit code rather than calling `std::process::exit` here
    /// keeps this function clean and lets `main` perform any future teardown.
    pub fn run(&self) -> i32 {
        let stdin = io::stdin();

        loop {
            self.print_prompt();

            let mut line = String::new();

            match stdin.lock().read_line(&mut line) {
                Ok(0) => {
                    // EOF: Ctrl-D in interactive mode, or end of a piped script.
                    if self.interactive {
                        println!();
                    }
                    return 0;
                }
                Ok(_)  => {}
                Err(e) => {
                    eprintln!("rush: read error: {e}");
                    return 1;
                }
            }

            match Command::parse(&line) {
                Err(Error::EmptyInput) => continue,
                Err(e)  => eprintln!("{e}"),
                Ok(cmd) => {
                    if let Err(e) = cmd.run(&self.resolver) {
                        eprintln!("{e}");
                    }
                }
            }
        }
    }

    /// Writes `$ ` to stdout and flushes — only in interactive mode.
    fn print_prompt(&self) {
        if self.interactive {
            print!("$ ");
            io::stdout().flush().ok();
        }
    }
}
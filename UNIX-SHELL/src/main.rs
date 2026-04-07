use unix_shell::shell::Shell;
use std::process::exit;

fn main() {
    let unix_shell = Shell::new();
    let exit_code = unix_shell.run();
    exit(exit_code);
}
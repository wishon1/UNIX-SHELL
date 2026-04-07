use crate::error::Error;

/// Exits the shell process.
///
/// Parses `args[0]` as an `i32` exit code; default to `0`.
/// Never returns - `std::process::exit` terminates the process immediately.
pub fn run(args: &[String]) -> Result<(), Error> {
    let code = args
        .first()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);

    std::process::exit(code);
}
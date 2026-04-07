use crate::error::Error;

/// prints every enviroment variable to stdout as `KEY=VALUE`
pub fn run(_args: &[String]) -> Result<(), Error> {
    for (key, value) in std::env::vars() {
        println!("{key}={value}");
    }
    Ok({})
}
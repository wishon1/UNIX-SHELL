//! Integration tests for `unix_shell`.
//!
//! All tests live here — no #[cfg(test)] blocks inside src/.
//! This file is compiled as a separate crate that imports the library
//! exactly as an external user would.

use unix_shell::command::Command;
use unix_shell::error::Error;
use unix_shell::path::PathResolver;

// ── Command::parse — valid input ──────────────────────────────────────────────

#[test]
fn parse_single_token_has_empty_args() {
    let cmd = Command::parse("ls").unwrap();
    assert_eq!(cmd.name, "ls");
    assert!(cmd.args.is_empty());
}

#[test]
fn parse_splits_name_from_args() {
    let cmd = Command::parse("ls -la /tmp").unwrap();
    assert_eq!(cmd.name, "ls");
    assert_eq!(cmd.args, vec!["-la", "/tmp"]);
}

#[test]
fn parse_many_args() {
    let cmd = Command::parse("cmd a b c d e").unwrap();
    assert_eq!(cmd.name, "cmd");
    assert_eq!(cmd.args, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn parse_leading_whitespace_is_ignored() {
    let cmd = Command::parse("   ls").unwrap();
    assert_eq!(cmd.name, "ls");
}

#[test]
fn parse_trailing_whitespace_is_ignored() {
    let cmd = Command::parse("ls   ").unwrap();
    assert_eq!(cmd.name, "ls");
    assert!(cmd.args.is_empty());
}

#[test]
fn parse_multiple_spaces_between_tokens_are_normalised() {
    let cmd = Command::parse("echo   hello   world").unwrap();
    assert_eq!(cmd.name, "echo");
    assert_eq!(cmd.args, vec!["hello", "world"]);
}

#[test]
fn parse_tabs_are_treated_as_whitespace() {
    let cmd = Command::parse("git\tlog\t--oneline").unwrap();
    assert_eq!(cmd.name, "git");
    assert_eq!(cmd.args, vec!["log", "--oneline"]);
}

#[test]
fn parse_newline_at_end_is_ignored() {
    // read_line includes the trailing newline — parse must handle it cleanly
    let cmd = Command::parse("ls -la\n").unwrap();
    assert_eq!(cmd.name, "ls");
    assert_eq!(cmd.args, vec!["-la"]);
}

#[test]
fn parse_absolute_path_as_command_name() {
    let cmd = Command::parse("/usr/bin/env FOO=bar").unwrap();
    assert_eq!(cmd.name, "/usr/bin/env");
    assert_eq!(cmd.args, vec!["FOO=bar"]);
}

#[test]
fn parse_relative_path_as_command_name() {
    let cmd = Command::parse("./myscript.sh arg1").unwrap();
    assert_eq!(cmd.name, "./myscript.sh");
    assert_eq!(cmd.args, vec!["arg1"]);
}

#[test]
fn parse_dash_flags_are_preserved_exactly() {
    let cmd = Command::parse("grep -r --include=*.rs pattern").unwrap();
    assert_eq!(cmd.args, vec!["-r", "--include=*.rs", "pattern"]);
}

#[test]
fn parse_arg_with_equals_sign_is_preserved() {
    let cmd = Command::parse("make VAR=value").unwrap();
    assert_eq!(cmd.args, vec!["VAR=value"]);
}

// ── Command::parse — invalid input ───────────────────────────────────────────

#[test]
fn parse_empty_string_is_empty_input_error() {
    assert!(matches!(Command::parse(""), Err(Error::EmptyInput)));
}

#[test]
fn parse_single_space_is_empty_input_error() {
    assert!(matches!(Command::parse(" "), Err(Error::EmptyInput)));
}

#[test]
fn parse_whitespace_only_is_empty_input_error() {
    assert!(matches!(Command::parse("   \t\n"), Err(Error::EmptyInput)));
}

#[test]
fn parse_newline_only_is_empty_input_error() {
    // what read_line returns when the user presses enter on a blank line
    assert!(matches!(Command::parse("\n"), Err(Error::EmptyInput)));
}

// ── Command fields — ownership and structure ──────────────────────────────────

#[test]
fn parse_name_is_owned_string_not_a_slice() {
    // if name were &str this would not compile — proves ownership is correct
    let name: String = Command::parse("ls").unwrap().name;
    assert_eq!(name, "ls");
}

#[test]
fn parse_args_are_owned_strings() {
    let args: Vec<String> = Command::parse("ls -la").unwrap().args;
    assert_eq!(args, vec!["-la"]);
}

#[test]
fn two_identical_parses_are_equal() {
    let a = Command::parse("ls -la").unwrap();
    let b = Command::parse("ls -la").unwrap();
    assert_eq!(a, b);
}

#[test]
fn two_different_parses_are_not_equal() {
    let a = Command::parse("ls -la").unwrap();
    let b = Command::parse("ls -l").unwrap();
    assert_ne!(a, b);
}

// ── PathResolver — construction ───────────────────────────────────────────────

#[test]
fn resolver_constructs_without_panicking() {
    // PATH may or may not be set — new() must never panic
    let _ = PathResolver::new();
}

// ── PathResolver — known executables ─────────────────────────────────────────

#[test]
fn resolver_finds_ls() {
    assert!(PathResolver::new().resolve("ls").is_some());
}

#[test]
fn resolver_finds_sh() {
    assert!(PathResolver::new().resolve("sh").is_some());
}

#[test]
fn resolver_finds_echo() {
    assert!(PathResolver::new().resolve("echo").is_some());
}

#[test]
fn resolver_finds_cat() {
    assert!(PathResolver::new().resolve("cat").is_some());
}

#[test]
fn resolver_finds_pwd() {
    assert!(PathResolver::new().resolve("pwd").is_some());
}

// ── PathResolver — unknown and invalid input ──────────────────────────────────

#[test]
fn resolver_returns_none_for_unknown_command() {
    assert!(PathResolver::new().resolve("__no_such_cmd__").is_none());
}

#[test]
fn resolver_returns_none_for_empty_string() {
    assert!(PathResolver::new().resolve("").is_none());
}

#[test]
fn resolver_returns_none_for_bad_absolute_path() {
    assert!(PathResolver::new().resolve("/no/such/binary").is_none());
}

// ── PathResolver — path shape ─────────────────────────────────────────────────

#[test]
fn resolver_handles_absolute_path_directly() {
    assert!(PathResolver::new().resolve("/bin/sh").is_some());
}

#[test]
fn resolver_resolved_path_ends_with_command_name() {
    let resolved = PathResolver::new().resolve("ls").unwrap();
    assert!(resolved.ends_with("ls"));
}

#[test]
fn resolver_resolved_path_is_absolute() {
    let resolved = PathResolver::new().resolve("ls").unwrap();
    assert!(resolved.starts_with('/'));
}

// ── Command::run — built-ins ──────────────────────────────────────────────────

#[test]
fn run_cd_to_tmp_succeeds() {
    let cmd = Command::parse("cd /tmp").unwrap();
    let resolver = PathResolver::new();
    assert!(cmd.run(&resolver).is_ok());
}

#[test]
fn run_cd_to_nonexistent_dir_returns_os_error() {
    let cmd = Command::parse("cd /no/such/dir").unwrap();
    let resolver = PathResolver::new();
    assert!(matches!(cmd.run(&resolver), Err(Error::Os(_))));
}

#[test]
fn run_env_succeeds() {
    let cmd = Command::parse("env").unwrap();
    let resolver = PathResolver::new();
    assert!(cmd.run(&resolver).is_ok());
}

// ── Command::run — external commands ─────────────────────────────────────────

#[test]
fn run_unknown_command_returns_command_not_found() {
    let cmd = Command::parse("__no_such_cmd__").unwrap();
    let resolver = PathResolver::new();
    assert!(matches!(
        cmd.run(&resolver),
        Err(Error::CommandNotFound(_))
    ));
}

#[test]
fn run_command_not_found_error_contains_the_name() {
    let cmd = Command::parse("__no_such_cmd__").unwrap();
    let resolver = PathResolver::new();
    match cmd.run(&resolver) {
        Err(Error::CommandNotFound(name)) => assert_eq!(name, "__no_such_cmd__"),
        other => panic!("expected CommandNotFound, got {:?}", other),
    }
}

#[test]
fn run_ls_succeeds() {
    let cmd = Command::parse("ls /tmp").unwrap();
    let resolver = PathResolver::new();
    assert!(cmd.run(&resolver).is_ok());
}

#[test]
fn run_echo_succeeds() {
    let cmd = Command::parse("echo hello").unwrap();
    let resolver = PathResolver::new();
    assert!(cmd.run(&resolver).is_ok());
}

// ── Error — display format ────────────────────────────────────────────────────

#[test]
fn error_command_not_found_display_contains_name() {
    let e = Error::CommandNotFound("foo".to_string());
    assert!(e.to_string().contains("foo"));
}

#[test]
fn error_os_display_contains_message() {
    let e = Error::Os("something failed".to_string());
    assert!(e.to_string().contains("something failed"));
}

#[test]
fn error_empty_input_display_is_empty() {
    let e = Error::EmptyInput;
    assert!(e.to_string().is_empty());
}
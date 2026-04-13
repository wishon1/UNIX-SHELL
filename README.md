# rush

A Unix-like shell written in Rust.

## Project layout

```
rush/
├── Cargo.toml
├── LICENSE
├── README.md
├── src/
│   ├── main.rs          # Entry point — constructs Shell, passes exit code to process::exit
│   ├── lib.rs           # Crate root — module declarations and crate-level docs
│   ├── shell.rs         # Shell struct — owns the REPL, PathResolver, and I/O state
│   ├── command.rs       # Command struct — parse() and run()
│   ├── path.rs          # PathResolver struct — reads PATH once, resolves on demand
│   ├── error.rs         # Error enum — implements std::error::Error
│   └── builtin/
│       ├── mod.rs       # Builtin enum + lookup() — zero-allocation dispatch
│       ├── cd.rs        # cd implementation
│       ├── env.rs       # env implementation
│       └── exit.rs      # exit implementation
└── tests/
    └── integration.rs   # Integration tests against the public library API
```

## Design decisions

| Decision | Reason |
|---|---|
| `lib.rs` + `main.rs` | All logic is in the library; the binary is a two-line launch pad. The library is independently testable. |
| `Shell` owns `PathResolver` | `PATH` is read once at startup, not on every command. |
| `Shell::run()` returns `i32` | The loop does not call `process::exit` directly — it returns a code to `main`, which exits cleanly. |
| `Command::parse()` not `new()` | `new()` in Rust convention is infallible construction. This is a fallible parse of external input. |
| `Command::run()` takes `&PathResolver` | Dependency is passed in explicitly — no hidden global state reads. |
| `Builtin` is an enum | Enum dispatch costs nothing. `Box<dyn Trait>` allocates a heap object per command for no benefit. |
| `tests/` directory | Integration tests are separate from unit tests. They test the public API as a user would. |
| `Error` implements `std::error::Error` | Required for interoperability with the broader Rust error-handling ecosystem. |

## Built-ins

| Command | Behaviour |
|---|---|
| `cd [dir]` | Change directory; defaults to `$HOME` when no argument given |
| `env` | Print all environment variables as `KEY=VALUE` |
| `exit [code]` | Exit with optional status code; defaults to `0` |

## Build

```bash
cargo build --release
./target/release/rush
```

## Test

```bash
cargo test
```
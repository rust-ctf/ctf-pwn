# ctf-pwn

Runtime-agnostic async I/O primitives for CTF challenges, exploit development, and hardware hacking.

Supports `tokio`, `async-std`, and `embassy` runtimes via feature flags. Core logic is `no_std + alloc` compatible.

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `std` | Standard library support | ✅ |
| `alloc` | Heap allocation (Vec buffers) | ✅ (via `std`) |
| `runtime-tokio` | Tokio runtime (TCP, process, timers) | ✅ |
| `runtime-async-std` | async-std runtime (TCP, process, timers) | |
| `runtime-embassy` | Embassy runtime (UART, USB CDC, timers) | |
| `regex` | Regex-based read operations | ✅ |
| `tui` | Interactive terminal UI (crossterm/ratatui) | |

## Building

```bash
# Default (tokio + regex)
cargo build

# Specific runtime
cargo build --no-default-features --features "runtime-tokio,regex"
cargo build --no-default-features --features "runtime-async-std,regex"
cargo build --no-default-features --features "runtime-embassy"

# no_std core only (no runtime)
cargo build --no-default-features --features alloc

# All features
cargo build --all-features
```

## Testing

All commands treat warnings as errors via `RUSTFLAGS="-D warnings"`.

```bash
# Run tests with default features (tokio)
RUSTFLAGS="-D warnings" cargo test

# Test per runtime
RUSTFLAGS="-D warnings" cargo test --no-default-features --features "runtime-tokio,regex"
RUSTFLAGS="-D warnings" cargo test --no-default-features --features "runtime-async-std,regex"
RUSTFLAGS="-D warnings" cargo test --no-default-features --features "runtime-embassy,regex"

# Test all features
RUSTFLAGS="-D warnings" cargo test --all-features
```

## Linting

```bash
# Clippy with warnings as errors
RUSTFLAGS="-D warnings" cargo clippy --all-features

# Check no_std compiles
RUSTFLAGS="-D warnings" cargo check --no-default-features --features alloc
```

## License

MIT

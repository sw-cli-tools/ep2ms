# ep2ms

Convert the current time to milliseconds since the Unix epoch.

## Overview

`ep2ms` is a small Rust CLI that prints the number of milliseconds elapsed
since 1970-01-01T00:00:00Z (the Unix epoch) for the current UTC time.

Typical uses include cache-busting query parameters, unique ID generation,
and measuring elapsed wall-clock time between two shell invocations.

## Installation

**Prerequisites**: Rust toolchain (1.85+ for edition 2024)

```bash
git clone <repository-url>
cd ep2ms
cargo build --release
# Binary will be at ./target/release/ep2ms
```

Or install with `sw-install`:

```bash
sw-install -p /path/to/ep2ms
```

## Usage

```bash
# Get current epoch milliseconds
ep2ms
# 1773941677565

# Show the ISO 8601 timestamp that produced the result
ep2ms -v
# stderr: 2026-03-19T17:34:37.571790+00:00
# stdout: 1773941677571

# Version information
ep2ms -V
# ep2ms 0.1.0
# Copyright (c) 2026 Michael A Wright
# License: MIT
# Repository: https://github.com/softwarewrighter/ep2ms
#
# Build Information:
#   Host: manager
#   Commit: 23df14e
#   Timestamp: 2026-03-19T10:39:25-0700

# Short help
ep2ms -h

# Extended help (includes AI coding agent instructions)
ep2ms --help
```

## Flags

| Flag | Long | Description |
|------|------|-------------|
| `-h` | `--help` | Print help (`-h` short, `--help` extended with AI agent guidance) |
| `-V` | `--version` | Show version and license information |
| `-v` | `--verbose` | Print the ISO 8601 / RFC 3339 UTC timestamp to stderr |

## Output Format

- **stdout**: a single line containing the integer milliseconds since epoch.
- **stderr** (with `-v`/`--verbose`): the ISO 8601 / RFC 3339 UTC timestamp
  used to produce the result.

## Examples

### Cache-busting URLs

```bash
echo "https://example.com/style.css?ts=$(ep2ms)"
# https://example.com/style.css?ts=1773941677565
```

### Measuring elapsed time

```bash
START=$(ep2ms)
sleep 2
END=$(ep2ms)
echo "Elapsed: $((END - START)) ms"
# Elapsed: 2003 ms
```

### Verbose output

```bash
ep2ms -v
# 2026-03-19T17:34:37.571790+00:00   (stderr)
# 1773941677571                        (stdout)
```

## AI Coding Agent Help

The `--help` flag includes an extended section with AI coding agent
instructions. This provides usage patterns, output format details, and
exit codes to help AI agents use the tool effectively. Use `-h` for the
short summary.

## Development

### Building

```bash
cargo build          # debug
cargo build --release  # release
```

### Code Quality

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
```

### Checkpoint Process

1. Run tests: `cargo test`
2. Fix clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
3. Format code: `cargo fmt --all`
4. Validate markdown: `markdown-checker -f "**/*.md"`
5. Run checklist: `sw-checklist`
6. Update documentation as needed
7. Commit and push

## License

MIT - see [LICENSE](LICENSE) for details.

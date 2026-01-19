# AGENTS.md

## Scope
- Repository: `gittracker-rs` (Rust CLI)
- Primary entrypoint: `src/main.rs`
- No existing Cursor rules or Copilot instructions found.

## Build, Lint, Test

### Build
- Build debug binary: `cargo build`
- Build release binary: `cargo build --release`
- Run the CLI locally: `cargo run -- <args>`

### Lint / Format
- Format codebase: `cargo fmt`
- Check formatting only: `cargo fmt -- --check`
- Run clippy lints: `cargo clippy`
- Clippy with warnings as errors: `cargo clippy -- -D warnings`

### Tests
- Run all tests: `cargo test`
- Run a single unit test: `cargo test <test_name>`
- Run tests in one module: `cargo test <module_path>::` (prefix match)
- Run tests in one file: `cargo test --test <integration_test_file>`
- Show test output: `cargo test -- --nocapture`

### Examples
- Scan current directory: `cargo run -- .`
- Scan and output JSON: `cargo run -- --json .`
- Include clean repos: `cargo run -- --show-clean .`

## Code Style Guidelines

### Rust Version and Tooling
- Use Rust 2024 edition conventions.
- Prefer stable Rust features only.
- Keep dependencies minimal and purposeful.

### Formatting
- Use `rustfmt` defaults; avoid manual alignment.
- Keep lines readable; wrap when rustfmt does.
- Use trailing commas in multi-line structs/enums.
- Keep blank lines between logical sections.

### Imports
- Order imports as: standard library, external crates, local modules.
- Group `std` imports together using braces when needed.
- Avoid unused imports; clippy should be clean.
- Prefer explicit imports over glob imports.

### Naming
- Types/structs/enums: `PascalCase`.
- Functions/variables/fields: `snake_case`.
- Constants: `SCREAMING_SNAKE_CASE`.
- Boolean fields: prefix with `is_`, `has_`, `should_`.

### Types and Ownership
- Prefer borrowing (`&T`) over cloning when possible.
- Use `Path`/`PathBuf` for filesystem paths.
- Avoid `String` allocations when `&str` suffices.
- Use `usize` for counters and collection lengths.

### Error Handling
- Prefer returning `Result<T, E>` from helpers.
- Handle command execution errors explicitly.
- Use `unwrap_or_else` only when a fallback is safe.
- Avoid `expect`/`unwrap` in non-test code.

### CLI and Output
- Keep CLI flags in `clap` derive structs.
- Document CLI arguments with doc comments.
- Human-readable output should be concise.
- JSON output should be stable and schema-like.

### Control Flow
- Keep loops focused; exit early with `continue`.
- Use `match` for branching on fallible results.
- Avoid deeply nested conditionals; refactor if needed.

### Data Structures
- Prefer plain structs for serialized output.
- Use `serde::Serialize` for JSON output types.
- Keep structs small and single-purpose.

### External Commands
- Use `Command` with explicit args and `-C`.
- Avoid shelling out with `sh -c`.
- Treat command output as UTF-8 lossy if needed.

### Performance
- Keep directory walks streaming and non-recursive where possible.
- Avoid extra allocations inside tight loops.
- Prefer iterators when readable.

### Safety
- Skip unreadable directories instead of failing.
- Do not follow symlinks unless required.
- Ensure `.git` traversal is skipped once detected.

### Testing Guidance
- Add unit tests for parsing functions and output formatting.
- Keep tests deterministic; avoid hitting the network.
- Use temp directories for filesystem tests.

### Documentation
- Keep public behavior described in CLI help.
- Update this file when tooling or conventions change.

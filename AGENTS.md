# Repository Guidelines

## Project Structure & Module Organization
- `Cargo.toml` defines the workspace with `sfizz-sys` (raw FFI) and `sfizz` (safe wrapper).
- `sfizz-sys/` contains the `build.rs` CMake driver plus generated bindings in `src/lib.rs`.
- `sfizz/` exposes the Rust-facing API; integration tests live in `sfizz/tests/` and examples under `examples/`.
- `vendor/sfizz/` is a git submodule tracking the upstream C++ sources.
- `fixtures/` holds optional SFZ sample libraries (ignored by git); add README updates there instead of committing audio assets.

## Build, Test, and Development Commands
- `cargo build -p sfizz` – builds the safe wrapper and triggers the native CMake pipeline.
- `cargo test -p sfizz` – runs unit tests plus the Taiko integration test (requires `fixtures/sfz/SCC Taiko Drums`).
- `cargo fmt` – formats Rust sources using rustfmt; run before commits.

## Coding Style & Naming Conventions
- Rust code follows the 2021 edition with rustfmt defaults (4-space indentation, snake_case functions, UpperCamelCase types).
- Avoid hand-editing generated bindings; they are emitted to `OUT_DIR`. Add comments sparingly to clarify complex blocks.
- CMake/C++ configuration stays in `sfizz-sys/build.rs`; keep options explicit and feature-gated.

## Testing Guidelines
- Rust tests use the built-in `#[test]` framework; integration cases go in `sfizz/tests/*.rs` with descriptive filenames.
- Prefer asserting on meaningful audio metrics (e.g., summed energy) rather than raw sample dumps.
- Large fixtures belong under `fixtures/` and should be referenced conditionally in tests to allow skipping when absent.

## Commit & Pull Request Guidelines
- Follow the existing style: short imperative subject (e.g., "Add safe sfizz wrapper and integration test"), optional body for context.
- Include summaries of native build changes or new assets in PR descriptions; mention required external dependencies.
- Reference related issues or tasks and note any follow-up work (e.g., additional wrappers or feature flags).

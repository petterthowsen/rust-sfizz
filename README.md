# sfizz-rust

Rust workspace that builds and wraps the [sfizz](https://github.com/sfztools/sfizz) SFZ synthesizer for use in DAWs.

## Layout

- `sfizz-sys/` – low-level FFI crate that builds the native sfizz C API and exposes raw bindings.
- `sfizz/` – safe, ergonomic wrapper crate for Rust applications.
- `vendor/sfizz/` – git checkout of the upstream C++ project (kept separate from the Rust sources).
- `bindings/` – optional home for generated bindings or manual patches.
- `examples/` – workspace-wide example binaries.
- `tests/` – cross-crate integration tests.

## Getting Started

### Prerequisites

Make sure these tools and libraries are available on your system:

- Rust toolchain (recommended via [`rustup`](https://rustup.rs/))
- C++17 compiler (clang or gcc) and standard library headers
- CMake 3.16+
- `pkg-config`
- `libsndfile` development files (headers + library)
- Optional: `ninja` to speed up native builds

On Debian/Ubuntu you can install the native pieces with:

```sh
sudo apt install build-essential cmake pkg-config libsndfile1-dev ninja-build
```

### Build

```sh
git clone https://github.com/yourname/sfizz-rust
cd sfizz-rust
git submodule update --init --recursive
cargo build -p sfizz
```

## Development Notes

- `sfizz-sys/build.rs` drives the native build (CMake) and runs bindgen; edit it as upstream changes.
- Toggle optional features (e.g., `vendored`, `system`) to control how the native library is located. By default the `sfizz` crate builds the bundled sources via the submodule.
- End-to-end tests live under `tests/` and should exercise both crates together.

## License

Dual-licensed under MIT or Apache-2.0.

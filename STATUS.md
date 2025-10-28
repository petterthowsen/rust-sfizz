Summary
- Workspace root configured with shared metadata, README, and git submodule for `vendor/sfizz/`.
- Raw FFI crate builds the native library via CMake (static target) and runs bindgen with configurable features.
- Safe wrapper crate exposes an RAII `Synth` API while re-exporting raw bindings for advanced use.
- Minimal example binary plus placeholders for future integration tests and bindings artifacts.
- `cargo build -p sfizz` succeeds (after native toolchain install) and links against the vendored static library.

Next Steps
- Flesh out the safe wrapper surface and add integration tests that exercise loading/playing SFZ.
- Consider feature flags for optional components (e.g. enabling audiofile dependencies, shared library builds).

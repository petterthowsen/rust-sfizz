Summary
- Workspace root configured with shared metadata and README.
- Raw FFI crate added with build script driving CMake/bindgen and optional system linking.
- Safe wrapper crate exposing RAII `Synth` API and re-exported raw bindings.
- Minimal example binary plus placeholders for future integration tests and bindings notes.
- Upstream C++ sources cloned into `vendor/sfizz/` for local builds.

Next Steps
- Install native build prerequisites and run `cargo build -p sfizz`.
- Optionally convert `vendor/sfizz` to a git submodule and expand wrapper/tests once builds succeed.

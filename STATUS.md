Summary
- Workspace root configured with shared metadata, README, and git submodule for `vendor/sfizz/`.
- Raw FFI crate builds the native library via CMake (aggregating static archives) and runs bindgen with configurable features.
- Safe wrapper crate exposes an RAII `Synth` API with sample-rate/block-size control, note events, and render helpers, while re-exporting raw bindings.
- Minimal example binary plus integration test exercising a real SFZ instrument under `fixtures/sfz/SCC Taiko Drums`.
- `cargo build -p sfizz` and `cargo test -p sfizz` succeed (after native toolchain install) and link against the vendored static library.

Next Steps
- Continue broadening the safe wrapper surface (additional MIDI/CC helpers, streaming controls).
- Consider feature flags for optional components (e.g. enabling audiofile dependencies, shared library builds) and add more DAW-focused smoke tests.

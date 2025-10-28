# Fixtures

Integration tests expect optional SFZ content at `fixtures/sfz/`.

This directory is `.gitignore`d to avoid committing large sample libraries. To run the Taiko integration test, copy the `SCC Taiko Drums` library (or any other suitable SFZ) into `fixtures/sfz/` before running `cargo test -p sfizz`.

//! High-level Rust wrapper around the sfizz synthesizer C API.

use sfizz_sys::bindings;
use std::ffi::{CString, NulError};
use std::path::Path;
use std::ptr::NonNull;

/// Re-export raw bindings for advanced use cases.
pub use bindings::*;

/// Errors that can occur when working with the sfizz wrapper.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sfizz failed to allocate a synthesizer instance")]
    CreationFailed,
    #[error("sfizz failed to load file: {path}")]
    LoadFailed { path: String },
    #[error("path is not valid UTF-8")]
    InvalidPath,
    #[error("path contains an interior NUL byte")]
    InteriorNul(#[from] NulError),
}

/// Managed wrapper around `sfizz_synth_t`.
pub struct Synth {
    raw: NonNull<bindings::sfizz_synth_t>,
}

impl Synth {
    /// Create a new synth instance with default settings.
    pub fn new() -> Result<Self, Error> {
        let ptr = unsafe { bindings::sfizz_create_synth() };
        let raw = NonNull::new(ptr).ok_or(Error::CreationFailed)?;
        Ok(Self { raw })
    }

    /// Load an `.sfz` file into the synthesizer.
    pub fn load_sfz<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let path_ref = path.as_ref();
        let path_str = path_ref.to_str().ok_or(Error::InvalidPath)?.to_owned();
        let c_path = CString::new(path_str.clone())?;

        let ok = unsafe { bindings::sfizz_load_file(self.raw.as_ptr(), c_path.as_ptr()) };
        if ok {
            Ok(())
        } else {
            Err(Error::LoadFailed { path: path_str })
        }
    }

    /// Access the raw pointer for advanced integrations.
    pub fn as_raw(&self) -> *mut bindings::sfizz_synth_t {
        self.raw.as_ptr()
    }
}

impl Drop for Synth {
    fn drop(&mut self) {
        unsafe {
            bindings::sfizz_free(self.raw.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synth_creation_is_safe() {
        // Creation may fail if the native library is not present, but should never panic.
        let _ = Synth::new();
    }
}

//! High-level Rust wrapper around the sfizz synthesizer C API.

use sfizz_sys::bindings;
use std::convert::TryFrom;
use std::ffi::{CStr, CString, NulError};
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
    #[error("requested block size {samples} exceeds i32::MAX")]
    BlockSizeTooLarge { samples: usize },
    #[error("render requested {frames} frames which exceeds i32::MAX")]
    FrameCountTooLarge { frames: usize },
    #[error("render requested {channels} channels which exceeds i32::MAX")]
    ChannelCountTooLarge { channels: usize },
    #[error("all output channels must have the same length")]
    ChannelLengthMismatch,
    #[error("at least one output channel is required")]
    NoChannels,
}

/// Managed wrapper around `sfizz_synth_t`.
pub struct Synth {
    raw: NonNull<bindings::sfizz_synth_t>,
}

/// Describes a labeled MIDI CC exposed by the current SFZ instrument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CcLabel {
    pub cc_number: u8,
    pub name: String,
}

impl Synth {
    /// Create a new synth instance with default settings.
    pub fn new() -> Result<Self, Error> {
        let ptr = unsafe { bindings::sfizz_create_synth() };
        let raw = NonNull::new(ptr).ok_or(Error::CreationFailed)?;
        Ok(Self { raw })
    }

    /// Set the sample rate used for rendering (in Hz).
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        unsafe { bindings::sfizz_set_sample_rate(self.raw.as_ptr(), sample_rate) };
    }

    /// Set the maximum number of frames processed per render call.
    pub fn set_block_size(&mut self, samples: usize) -> Result<(), Error> {
        let samples_i32 =
            i32::try_from(samples).map_err(|_| Error::BlockSizeTooLarge { samples })?;
        unsafe { bindings::sfizz_set_samples_per_block(self.raw.as_ptr(), samples_i32) };
        Ok(())
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

    /// Trigger a note-on event with MIDI velocity (0-127).
    pub fn note_on(&mut self, note: u8, velocity: u8) {
        unsafe {
            bindings::sfizz_send_note_on(self.raw.as_ptr(), 0, note as i32, velocity as i32);
        }
    }

    /// Trigger a note-off event with MIDI release velocity (0-127).
    pub fn note_off(&mut self, note: u8, velocity: u8) {
        unsafe {
            bindings::sfizz_send_note_off(self.raw.as_ptr(), 0, note as i32, velocity as i32);
        }
    }

    /// Immediately silence all voices.
    pub fn all_sound_off(&mut self) {
        unsafe { bindings::sfizz_all_sound_off(self.raw.as_ptr()) };
    }

    /// Render a block of audio into the provided planar channel buffers.
    pub fn render_block(&mut self, outputs: &mut [&mut [f32]]) -> Result<(), Error> {
        if outputs.is_empty() {
            return Err(Error::NoChannels);
        }

        let frames = outputs[0].len();
        if outputs.iter().any(|channel| channel.len() != frames) {
            return Err(Error::ChannelLengthMismatch);
        }

        if frames > i32::MAX as usize {
            return Err(Error::FrameCountTooLarge { frames });
        }

        if outputs.len() > i32::MAX as usize {
            return Err(Error::ChannelCountTooLarge {
                channels: outputs.len(),
            });
        }

        let mut pointers: Vec<*mut f32> = outputs
            .iter_mut()
            .map(|channel| channel.as_mut_ptr())
            .collect();

        unsafe {
            bindings::sfizz_render_block(
                self.raw.as_ptr(),
                pointers.as_mut_ptr(),
                outputs.len() as i32,
                frames as i32,
            );
        }

        Ok(())
    }

    /// Access the raw pointer for advanced integrations.
    pub fn as_raw(&self) -> *mut bindings::sfizz_synth_t {
        self.raw.as_ptr()
    }

    /// Enumerate the labeled MIDI CCs declared in the currently loaded SFZ.
    pub fn cc_labels(&self) -> Vec<CcLabel> {
        let count = unsafe { bindings::sfizz_get_num_cc_labels(self.raw.as_ptr()) } as usize;
        let mut labels = Vec::with_capacity(count);

        for index in 0..count {
            let number =
                unsafe { bindings::sfizz_get_cc_label_number(self.raw.as_ptr(), index as i32) };
            if number == bindings::SFIZZ_OUT_OF_BOUNDS_LABEL_INDEX {
                continue;
            }

            let text_ptr =
                unsafe { bindings::sfizz_get_cc_label_text(self.raw.as_ptr(), index as i32) };
            if text_ptr.is_null() {
                continue;
            }

            let name = unsafe { CStr::from_ptr(text_ptr) }
                .to_string_lossy()
                .into_owned();

            let cc_number = match u8::try_from(number) {
                Ok(value) => value,
                Err(_) => continue,
            };

            labels.push(CcLabel { cc_number, name });
        }

        labels
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

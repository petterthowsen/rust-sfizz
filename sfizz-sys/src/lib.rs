#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(clippy::all)]

/// Raw bindings generated from the sfizz C API headers.
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/sfizz_bindings.rs"));
}

pub use bindings::*;

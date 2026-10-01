// src/steamaudio/mod.rs

pub(in crate::steamaudio_module) mod binds;
mod errors;
mod ffi;

pub(in crate::steamaudio_module) use binds::*;
use errors::*;
pub use ffi::*;

pub const STEAMAUDIO_VERSION: u32 =
    (STEAMAUDIO_VERSION_MAJOR << 16) | (STEAMAUDIO_VERSION_MINOR << 8) | STEAMAUDIO_VERSION_PATCH;

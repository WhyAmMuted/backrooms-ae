// src/steamaudio/mod.rs
#![allow(dead_code)]
#![allow(unused)]

pub mod basic;
pub(in crate::steamaudio_module) mod binds;
mod errors;
mod ffi;
mod simd_level;

pub use binds::iplAudioBufferAllocate;
pub(in crate::steamaudio_module) use binds::*;
pub use errors::*;
pub use ffi::*;
pub use simd_level::*;

pub const STEAMAUDIO_VERSION: u32 =
    (STEAMAUDIO_VERSION_MAJOR << 16) | (STEAMAUDIO_VERSION_MINOR << 8) | STEAMAUDIO_VERSION_PATCH;

pub type Vector3 = IPLVector3;

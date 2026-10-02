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

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn convert_to_ipl(&self) -> IPLVector3 {
        IPLVector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

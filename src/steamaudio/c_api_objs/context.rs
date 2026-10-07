use std::ptr::null_mut;

use crate::steamaudio::{
    bind::{
        IPLContext, IPLContextSettings, IPLSIMDLevel_IPL_SIMDLEVEL_AVX2, STEAMAUDIO_VERSION,
        iplContextCreate, iplContextRelease,
    },
    errors::Status,
};

pub struct Context {
    context: IPLContext,
}

impl Context {
    pub fn new() -> Result<Self, Status> {
        let mut context = null_mut();
        let status = unsafe {
            iplContextCreate(
                &mut IPLContextSettings {
                    version: STEAMAUDIO_VERSION,
                    logCallback: None,
                    allocateCallback: None,
                    freeCallback: None,
                    simdLevel: IPLSIMDLevel_IPL_SIMDLEVEL_AVX2,
                    flags: 0,
                },
                &mut context,
            )
        };
        Status::catch(Self { context: context }, Status::from(status))
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            iplContextRelease(&mut self.context);
        }
    }
}

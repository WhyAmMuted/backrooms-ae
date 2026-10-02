use std::ptr::null_mut;

use crate::steamaudio_module::{
    IPLAllocateFunction, IPLContext, IPLContextFlags, IPLContextSettings, IPLFreeFunction,
    IPLLogFunction, SimdLevels, SteamAudioErrors, catch_error, iplContextCreate, iplContextRelease,
};

#[derive(Clone, Copy)]
pub struct ContextSettings {
    version: u32,
    log_callback: IPLLogFunction,
    allocate_callback: IPLAllocateFunction,
    free_callback: IPLFreeFunction,
    simd_level: SimdLevels,
    flags: IPLContextFlags,
}

impl ContextSettings {
    pub fn new(version: u32, simd_level: SimdLevels) -> ContextSettings {
        let mut context = ContextSettings {
            version: version,
            log_callback: None,
            allocate_callback: None,
            free_callback: None,
            flags: 0,
            simd_level: simd_level,
        };

        context
    }

    pub(in crate::steamaudio_module) fn convert(&self) -> IPLContextSettings {
        IPLContextSettings {
            version: self.version,
            logCallback: self.log_callback,
            allocateCallback: self.allocate_callback,
            freeCallback: self.free_callback,
            simdLevel: self.simd_level.convert(),
            flags: self.flags,
        }
    }
}

pub struct Context {
    context_settings: ContextSettings,
    context: IPLContext,
}

impl Context {
    pub fn new(_context_settings: ContextSettings) -> Result<Context, SteamAudioErrors> {
        let mut context: IPLContext = null_mut();
        let status = unsafe { iplContextCreate(&mut _context_settings.convert(), &mut context) };
        let status = SteamAudioErrors::convert(status);

        catch_error(
            status,
            Context {
                context_settings: _context_settings,
                context,
            },
        )
    }

    pub fn as_raw(&self) -> IPLContext {
        self.context
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            iplContextRelease(&mut self.context);
        }
    }
}

use std::ptr::null_mut;

use crate::steamaudio_module::{
    IPLAudioBuffer, IPLContext, SteamAudioErrors, basic::context::Context, catch_error,
    iplAudioBufferAllocate, iplAudioBufferDeinterleave, iplAudioBufferFree,
    iplAudioBufferInterleave,
};

pub struct AudioBuffer {
    audio_buffer: IPLAudioBuffer,
    context: IPLContext,
}

impl AudioBuffer {
    pub fn new(
        num_channels: i32,
        num_samples: i32,
        context: &Context,
    ) -> Result<AudioBuffer, SteamAudioErrors> {
        let mut audio_buffer = AudioBuffer {
            audio_buffer: IPLAudioBuffer {
                numChannels: num_channels,
                numSamples: num_samples,
                data: null_mut(),
            },
            context: context.as_raw(),
        };

        let status = unsafe {
            iplAudioBufferAllocate(
                context.as_raw(),
                num_channels,
                num_samples,
                audio_buffer.as_raw_mut(),
            )
        };
        let status = SteamAudioErrors::convert(status);
        catch_error(status, audio_buffer)
    }

    pub fn interleave(&mut self, dst: *mut f32) {
        unsafe {
            iplAudioBufferInterleave(self.context, self.as_raw_mut(), dst);
        }
    }

    pub fn deinterleave(&mut self, src: &[f32]) {
        unsafe {
            iplAudioBufferDeinterleave(self.context, src.as_ptr() as *mut f32, self.as_raw_mut());
        }
    }

    pub fn add_data(self, data: *mut *mut f32) -> AudioBuffer {
        let mut audio_buffer = self;
        audio_buffer.audio_buffer.data = data;
        audio_buffer
    }

    pub fn as_raw(&self) -> &IPLAudioBuffer {
        &self.audio_buffer
    }

    pub fn as_raw_mut(&mut self) -> &mut IPLAudioBuffer {
        &mut self.audio_buffer
    }

    pub fn num_channels(&self) -> i32 {
        self.audio_buffer.numChannels
    }

    pub fn num_samples(&self) -> i32 {
        self.audio_buffer.numSamples
    }

    pub fn channel(&self, index: usize) -> &[f32] {
        unsafe {
            let ptr = *&self.audio_buffer.data.add(index);
            std::slice::from_raw_parts(ptr as *const f32, self.num_samples() as usize)
        }
    }
}

impl Drop for AudioBuffer {
    fn drop(&mut self) {
        if self.audio_buffer.data.is_null() {
            return;
        }
        unsafe {
            iplAudioBufferFree(self.context, &mut self.audio_buffer);
        }
    }
}

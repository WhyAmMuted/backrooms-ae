use std::{ptr::null_mut, sync::Arc};

use crate::steamaudio::{
    bind::{
        IPLAudioBuffer, iplAudioBufferAllocate, iplAudioBufferDeinterleave, iplAudioBufferFree,
        iplAudioBufferInterleave,
    },
    c_api_objs::context::Context,
    errors::Status,
};

pub struct AudioBuffer {
    buffer: IPLAudioBuffer,
    context: Arc<Context>,
}

impl AudioBuffer {
    pub fn new(stereo: bool, count_of_samples: i32, context: Arc<Context>) -> Result<Self, Status> {
        let mut channels = 1;
        if stereo {
            channels = 2;
        }
        let buffer = AudioBuffer {
            buffer: IPLAudioBuffer {
                numChannels: channels,
                numSamples: count_of_samples,
                data: null_mut(),
            },
            context: context.clone(),
        };

        let status = unsafe {
            iplAudioBufferAllocate(
                context.as_raw(),
                channels,
                count_of_samples,
                &mut buffer.as_raw(),
            )
        };
        Status::catch(buffer, Status::from(status))
    }

    pub fn as_raw(&self) -> IPLAudioBuffer {
        self.buffer
    }

    pub fn interleave(&mut self, dst: &[f32]) {
        unsafe {
            iplAudioBufferInterleave(
                self.context.as_raw(),
                &mut self.buffer,
                dst.as_ptr() as *mut f32,
            );
        }
    }

    pub fn deinterleave(&mut self, src: &[f32]) {
        unsafe {
            iplAudioBufferDeinterleave(
                self.context.as_raw(),
                src.as_ptr() as *mut f32,
                &mut self.buffer,
            );
        }
    }

    pub fn change_data(self, data: &[f32]) -> AudioBuffer {
        let mut audio_buffer = self;
        audio_buffer.buffer.data = data.as_ptr() as *mut *mut f32;
        audio_buffer
    }

    pub fn as_raw_mut(&mut self) -> &mut IPLAudioBuffer {
        &mut self.buffer
    }

    pub fn num_channels(&self) -> i32 {
        self.buffer.numChannels
    }

    pub fn num_samples(&self) -> i32 {
        self.buffer.numSamples
    }

    pub fn channel(&self, index: usize) -> &[f32] {
        unsafe {
            let ptr = *&self.buffer.data.add(index);
            std::slice::from_raw_parts(ptr as *const f32, self.num_samples() as usize)
        }
    }
}

impl Drop for AudioBuffer {
    fn drop(&mut self) {
        unsafe {
            iplAudioBufferFree(self.context.as_raw(), &mut self.buffer);
        }
    }
}

use std::{ptr::null_mut, sync::Arc};

use crate::steamaudio::{
    bind::{IPLHRTF, IPLHRTFSettings, iplHRTFCreate, iplHRTFRelease},
    c_api_objs::{context::Context, misc::audio_settings::AudioSettings},
    errors::Status,
};

pub struct HRTF {
    context: Arc<Context>,
    hrtf: IPLHRTF,
}

impl HRTF {
    pub fn new(
        volume: f32,
        context: Arc<Context>,
        audio_settings: AudioSettings,
    ) -> Result<Self, Status> {
        let mut hrtf = null_mut();
        let status = unsafe {
            iplHRTFCreate(
                context.as_raw(),
                &mut audio_settings.as_raw(),
                &mut IPLHRTFSettings {
                    volume: volume,
                    ..Default::default()
                },
                &mut hrtf,
            )
        };

        Status::catch(
            Self {
                context,
                hrtf: hrtf,
            },
            Status::from(status),
        )
    }

    pub fn as_raw(&self) -> IPLHRTF {
        self.hrtf
    }
}

impl Drop for HRTF {
    fn drop(&mut self) {
        unsafe {
            iplHRTFRelease(&mut self.hrtf);
        }
    }
}

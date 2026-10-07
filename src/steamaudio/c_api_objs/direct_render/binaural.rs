use std::{ptr::null_mut, sync::Arc};

use crate::steamaudio::{
    bind::{
        IPLBinauralEffect, IPLBinauralEffectSettings, iplBinauralEffectCreate,
        iplBinauralEffectRelease,
    },
    c_api_objs::{context::Context, misc::audio_settings::AudioSettings, spatial::hrtf::HRTF},
    errors::Status,
};

pub struct BinauralEffect {
    effect: IPLBinauralEffect,
}

impl BinauralEffect {
    pub fn new(
        context: Arc<Context>,
        audio_settings: &mut AudioSettings,
        hrtf: &mut HRTF,
    ) -> Result<Self, Status> {
        let mut effect = null_mut();
        let status = unsafe {
            iplBinauralEffectCreate(
                context.as_raw(),
                &mut audio_settings.as_raw(),
                &mut IPLBinauralEffectSettings {
                    hrtf: hrtf.as_raw(),
                },
                &mut effect,
            )
        };
        Status::catch(Self { effect: effect }, Status::from(status))
    }
}

impl Drop for BinauralEffect {
    fn drop(&mut self) {
        unsafe {
            iplBinauralEffectRelease(&mut self.effect);
        }
    }
}

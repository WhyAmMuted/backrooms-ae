use std::ptr::null_mut;

use crate::steamaudio_module::{
    IPLAudioEffectState, IPLBinauralEffect, IPLBinauralEffectParams, IPLBinauralEffectSettings,
    SteamAudioErrors, Vector3,
    basic::{
        audio_settings::AudioSettings,
        audiobuffer::AudioBuffer,
        context::Context,
        hrtf::{HRTF, HRTFInterpolation},
    },
    catch_error, iplBinauralEffectApply, iplBinauralEffectCreate, iplBinauralEffectRelease,
};

pub struct EffectSettings<'a> {
    hrtf: &'a HRTF,
}

impl<'a> EffectSettings<'a> {
    pub fn new(hrtf: &'a HRTF) -> Self {
        Self { hrtf }
    }

    pub fn get_effect(&self) -> IPLBinauralEffectSettings {
        IPLBinauralEffectSettings {
            hrtf: self.hrtf.as_raw(),
        }
    }
}

pub enum EffectState {
    Remaining,
    Complete,
}

impl EffectState {
    pub fn convert(&self) -> IPLAudioEffectState {
        match self {
            EffectState::Complete => 1,
            EffectState::Remaining => 0,
        }
    }
}

pub struct EffectParams<'a> {
    pub direction: Vector3,
    pub interpolation: HRTFInterpolation,
    pub spatial_blend: f32,
    pub hrtf: &'a HRTF,
    pub peak_delays: *mut f32,
}

impl EffectParams<'_> {
    pub fn convert(&self) -> IPLBinauralEffectParams {
        IPLBinauralEffectParams {
            direction: self.direction.convert_to_ipl(),
            interpolation: self.interpolation.convert(),
            spatialBlend: self.spatial_blend,
            hrtf: self.hrtf.as_raw(),
            peakDelays: self.peak_delays,
        }
    }
}

pub struct Effect {
    effect: IPLBinauralEffect,
}

impl Effect {
    pub fn new(
        effect_settings: EffectSettings,
        audio_settings: &mut AudioSettings,
        context: &Context,
    ) -> Result<Effect, SteamAudioErrors> {
        let mut effect: IPLBinauralEffect = null_mut();
        let status = unsafe {
            iplBinauralEffectCreate(
                context.as_raw(),
                &mut audio_settings.get_audio_settings(),
                &mut effect_settings.get_effect(),
                &mut effect,
            )
        };
        let status = SteamAudioErrors::convert(status);
        catch_error(status, Effect { effect })
    }

    pub fn as_raw(&self) -> IPLBinauralEffect {
        self.effect
    }

    pub fn apply(
        &self,
        params: &EffectParams,
        in_: &mut AudioBuffer,
        out: &mut AudioBuffer,
    ) -> EffectState {
        let status = unsafe {
            iplBinauralEffectApply(
                self.effect,
                &mut params.convert(),
                in_.as_raw_mut(),
                out.as_raw_mut(),
            )
        };

        match status {
            1 => EffectState::Complete,
            _ => EffectState::Remaining,
        }
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        unsafe {
            iplBinauralEffectRelease(&mut self.effect);
        }
    }
}

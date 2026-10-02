// src/steamaudio/ffi.rs

use crate::steamaudio_module::basic::{
    audiobuffer::AudioBuffer,
    effect::{self, Effect, EffectSettings},
    hrtf::{HRTF, HRTFSettings},
};

use super::*;

use std::ptr::{null, null_mut};

use basic::{
    audio_settings::AudioSettings,
    context::{Context, ContextSettings},
};
pub use binds::{
    IPLAudioBuffer, IPLBinauralEffectParams, IPLHRTFInterpolation_IPL_HRTFINTERPOLATION_NEAREST,
    IPLVector3, iplAudioBufferInterleave, iplBinauralEffectApply,
};
pub use simd_level::SimdLevels;

pub fn create_context(simd_level: SimdLevels) -> Result<Context, SteamAudioErrors> {
    let mut context_settings = ContextSettings::new(STEAMAUDIO_VERSION, simd_level);
    let context = Context::new(context_settings)?;

    Ok(context)
}

pub fn create_audio_settings(sample_rate: Option<i32>, frame_size: Option<i32>) -> AudioSettings {
    AudioSettings::new(sample_rate.unwrap_or(44100), frame_size.unwrap_or(1024))
}

pub fn create_hrtf(
    volume: Option<f32>,
    context: &Context,
    audio_settings: &mut AudioSettings,
) -> Result<HRTF, SteamAudioErrors> {
    let volume = volume.unwrap_or(1.0);

    let mut hrtf_settings = HRTFSettings::new(volume);

    let hrtf = HRTF::new(context, audio_settings, &mut hrtf_settings)?;

    Ok(hrtf)
}

pub fn create_binaural_effect(
    context: &Context,
    audio_settings: &mut AudioSettings,
    hrtf: &HRTF,
) -> Result<Effect, SteamAudioErrors> {
    let mut effect_settings = EffectSettings::new(&hrtf);
    let mut effect = Effect::new(effect_settings, audio_settings, &context)?;
    Ok(effect)
}

pub fn create_audio_buffer(
    audio_settings: &mut IPLAudioSettings,
    context: IPLContext,
    in_data: *mut *mut f32,
) -> Result<(IPLAudioBuffer, IPLAudioBuffer), SteamAudioErrors> {
    let mut in_buffer = IPLAudioBuffer {
        numChannels: 1,
        numSamples: audio_settings.frameSize,
        data: in_data,
    };

    let mut out_buffer: IPLAudioBuffer = in_buffer.clone();

    let status: IPLerror =
        unsafe { iplAudioBufferAllocate(context, 2, audio_settings.frameSize, &mut out_buffer) };
    catch_error(SteamAudioErrors::convert(status), (in_buffer, out_buffer))
}

pub fn free_all(context: Context, effect: Effect, hrtf: HRTF, audio_buffer: AudioBuffer) {
    unsafe {
        drop(audio_buffer);
        drop(effect);
        drop(hrtf);
        drop(context)
    }
}

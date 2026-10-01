// src/steamaudio/ffi.rs

use super::*;

use std::ptr::{null, null_mut};

pub use binds::{
    IPLSIMDLevel, IPLSIMDLevel_IPL_SIMDLEVEL_AVX, IPLSIMDLevel_IPL_SIMDLEVEL_AVX2,
    IPLSIMDLevel_IPL_SIMDLEVEL_AVX512, IPLSIMDLevel_IPL_SIMDLEVEL_NEON,
    IPLSIMDLevel_IPL_SIMDLEVEL_SSE2, IPLSIMDLevel_IPL_SIMDLEVEL_SSE4,
};
pub fn create_context(
    simd_level: IPLSIMDLevel,
) -> Result<(IPLContextSettings, IPLContext), IPLerror> {
    let mut context_settings = IPLContextSettings {
        version: STEAMAUDIO_VERSION,
        logCallback: None,
        allocateCallback: None,
        freeCallback: None,
        flags: 0,
        simdLevel: simd_level,
    };
    let mut context: IPLContext = null_mut();
    let status: IPLerror = unsafe { iplContextCreate(&mut context_settings, &mut context) };

    catch_error(status, (context_settings, context))
}

pub fn create_audio_settings(
    sample_rate: Option<i32>,
    frame_size: Option<i32>,
) -> IPLAudioSettings {
    let sample_rate = sample_rate.unwrap_or(44100);
    let frame_size = frame_size.unwrap_or(1024);

    let audio_settings = IPLAudioSettings {
        samplingRate: sample_rate,
        frameSize: frame_size,
    };

    audio_settings
}

pub fn create_hrtf(
    volume: Option<f32>,
    context: IPLContext,
    audio_settings: &mut IPLAudioSettings,
) -> Result<(IPLHRTFSettings, IPLHRTF), IPLerror> {
    let volume = volume.unwrap_or(1.0);
    let mut hrtf_settings = IPLHRTFSettings {
        type_: IPLHRTFType_IPL_HRTFTYPE_DEFAULT,
        volume,

        sofaData: null(),
        sofaDataSize: 0,
        sofaFileName: null(),
        normType: 0,
    };

    let mut hrtf: IPLHRTF = null_mut();
    let status = unsafe { iplHRTFCreate(context, audio_settings, &mut hrtf_settings, &mut hrtf) };
    catch_error(status, (hrtf_settings, hrtf))
}

pub fn create_binaural_effect(
    context: IPLContext,
    audio_settings: &mut IPLAudioSettings,
    hrtf: IPLHRTF,
) -> Result<(IPLBinauralEffectSettings, IPLBinauralEffect), IPLerror> {
    let mut effect_settings = IPLBinauralEffectSettings { hrtf: hrtf };

    let mut effect: IPLBinauralEffect = null_mut();

    let status: IPLerror = unsafe {
        iplBinauralEffectCreate(context, audio_settings, &mut effect_settings, &mut effect)
    };

    catch_error(status, (effect_settings, effect))
}

pub fn create_audio_buffer(
    audio_settings: &mut IPLAudioSettings,
    context: IPLContext,
    in_data: *mut *mut f32,
) -> Result<(IPLAudioBuffer, IPLAudioBuffer), IPLerror> {
    let mut in_buffer = IPLAudioBuffer {
        numChannels: 1,
        numSamples: audio_settings.frameSize,
        data: in_data,
    };

    let mut out_buffer: IPLAudioBuffer = in_buffer.clone();

    let status: IPLerror =
        unsafe { iplAudioBufferAllocate(context, 2, audio_settings.frameSize, &mut out_buffer) };
    catch_error(status, (in_buffer, out_buffer))
}

pub fn free_all(
    context: &mut IPLContext,
    effect: &mut IPLBinauralEffect,
    hrtf: &mut IPLHRTF,
    audio_buffer: &mut IPLAudioBuffer,
) {
    unsafe {
        iplAudioBufferFree(*context, audio_buffer);
        iplBinauralEffectRelease(effect);
        iplHRTFRelease(hrtf);
        iplContextRelease(context);
    }
}

use std::ptr::null_mut;

use crate::steamaudio_module::{
    IPLHRTF, IPLHRTFInterpolation, IPLHRTFNormType, IPLHRTFSettings, IPLHRTFType, SteamAudioErrors,
    basic::{
        audio_settings::{self, AudioSettings},
        context::{self, Context},
    },
    catch_error, iplHRTFCreate, iplHRTFRelease,
};

#[derive(Clone, Copy)]
pub enum HRTFType {
    Default,
    SOFA,
}

impl HRTFType {
    pub fn convert(&self) -> IPLHRTFType {
        match self {
            HRTFType::Default => 0,
            HRTFType::SOFA => 1,
        }
    }
}

#[derive(Clone, Copy)]
pub enum NormType {
    None,
    RMS,
}

impl NormType {
    pub fn convert(&self) -> IPLHRTFNormType {
        match self {
            NormType::None => 0,
            NormType::RMS => 1,
        }
    }
}

#[derive(Clone, Copy)]
pub enum HRTFInterpolation {
    Nearest,
    Bilinear,
}

impl HRTFInterpolation {
    pub fn convert(&self) -> IPLHRTFInterpolation {
        match self {
            HRTFInterpolation::Nearest => 0,
            HRTFInterpolation::Bilinear => 1,
        }
    }
}

#[derive(Clone, Copy)]
pub struct HRTFSettings {
    volume: f32,
    type_: HRTFType,
    norm_type: NormType,

    sofa_data: *const u8,
    sofa_data_size: i32,
    sofa_filename: *const i8,
}

impl HRTFSettings {
    pub fn new(volume: f32) -> HRTFSettings {
        HRTFSettings {
            volume: volume,
            type_: HRTFType::Default,
            norm_type: NormType::None,
            sofa_data: 0 as *const u8,
            sofa_data_size: 0,
            sofa_filename: 0 as *const i8,
        }
    }

    pub fn new_sofa() {
        // TODO
    }

    pub(in crate::steamaudio_module) fn convert(&self) -> IPLHRTFSettings {
        IPLHRTFSettings {
            type_: self.type_.convert(),
            sofaFileName: 0 as *const i8, // TODO: realise self.sofa_filename
            sofaData: self.sofa_data,
            sofaDataSize: self.sofa_data_size,
            volume: self.volume,
            normType: self.norm_type.convert(),
        }
    }
}

pub struct HRTF {
    hrtf_settings: HRTFSettings,
    hrtf: IPLHRTF,
}

impl HRTF {
    pub fn new(
        context: &Context,
        audio_settings: &mut AudioSettings,
        hrtf_settings: &mut HRTFSettings,
    ) -> Result<HRTF, SteamAudioErrors> {
        let mut hrtf = null_mut();
        let status = unsafe {
            iplHRTFCreate(
                context.as_raw(),
                &mut audio_settings.get_audio_settings(),
                &mut hrtf_settings.convert(),
                &mut hrtf,
            )
        };
        let status = SteamAudioErrors::convert(status);
        catch_error(
            status,
            HRTF {
                hrtf_settings: *hrtf_settings,
                hrtf: hrtf,
            },
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

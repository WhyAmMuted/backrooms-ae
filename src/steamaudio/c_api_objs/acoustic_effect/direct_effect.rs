use std::{ptr::null_mut, sync::Arc};

use crate::steamaudio::{
    bind::{
        IPLAudioEffectState_IPL_AUDIOEFFECTSTATE_TAILCOMPLETE, IPLDirectEffect,
        IPLDirectEffectFlags, IPLDirectEffectParams, IPLDirectEffectSettings, iplDirectEffectApply,
        iplDirectEffectCreate, iplDirectEffectGetTail, iplDirectEffectGetTailSize,
        iplDirectEffectReset,
    },
    c_api_objs::{
        buffers::AudioBuffer,
        context::Context,
        misc::{
            audio_params::{Absorption, Transmission},
            audio_settings::AudioSettings,
            direct_effect_params::{DirectEffectFlags, TransmissionType},
        },
    },
    errors::Status,
};

pub struct DirectEffect {
    effect: IPLDirectEffect,
}

impl DirectEffect {
    pub fn new(
        stereo: bool,
        audio_settings: &mut AudioSettings,
        context: Arc<Context>,
    ) -> Result<Self, Status> {
        let mut channels = 1;
        if stereo {
            channels = 2;
        }
        let mut effect = null_mut();
        let status = unsafe {
            iplDirectEffectCreate(
                context.as_raw(),
                &mut audio_settings.as_raw(),
                &mut IPLDirectEffectSettings {
                    numChannels: channels,
                },
                &mut effect,
            )
        };

        Status::catch(Self { effect: effect }, Status::from(status))
    }

    pub fn reset(&mut self) {
        unsafe {
            iplDirectEffectReset(self.effect);
        }
    }

    pub fn apply(&mut self, in_: &mut AudioBuffer, out_: &mut AudioBuffer, params: &mut DEParams) {
        unsafe {
            iplDirectEffectApply(
                self.effect,
                &mut params.as_raw(),
                in_.as_raw_mut(),
                out_.as_raw_mut(),
            )
        };
    }

    pub fn get_tail(&self, out_: &mut AudioBuffer) -> EffectStatus {
        let status = unsafe { iplDirectEffectGetTail(self.effect, out_.as_raw_mut()) };

        if status == IPLAudioEffectState_IPL_AUDIOEFFECTSTATE_TAILCOMPLETE {
            return EffectStatus::Complete;
        } else {
            return EffectStatus::Remaining;
        }
    }

    pub fn get_tail_size(&self) -> i32 {
        unsafe { iplDirectEffectGetTailSize(self.effect) }
    }
}
pub enum EffectStatus {
    Remaining,
    Complete,
}

pub struct DEParams {
    pub air_absorption: Absorption,
    pub transmission: Transmission,
    pub occlusion: f32,
    pub directivity: f32,
    pub flags: DirectEffectFlags,
    pub trans_type: TransmissionType,
    pub distance_attenuation: f32,
}

impl DEParams {
    pub fn as_raw(&self) -> IPLDirectEffectParams {
        IPLDirectEffectParams {
            flags: IPLDirectEffectFlags::from(self.flags),
            transmissionType: IPLDirectEffectFlags::from(self.trans_type),
            distanceAttenuation: self.distance_attenuation,
            airAbsorption: [
                self.air_absorption.low,
                self.air_absorption.middle,
                self.air_absorption.high,
            ],
            directivity: self.directivity,
            occlusion: self.occlusion,
            transmission: [
                self.transmission.low,
                self.transmission.middle,
                self.transmission.high,
            ],
        }
    }
}

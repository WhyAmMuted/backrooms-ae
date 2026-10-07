use crate::steamaudio::bind::{
    IPLDirectEffectFlags, IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYAIRABSORPTION,
    IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYDIRECTIVITY,
    IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYDISTANCEATTENUATION,
    IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYOCCLUSION,
    IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYTRANSMISSION, IPLTransmissionType,
    IPLTransmissionType_IPL_TRANSMISSIONTYPE_FREQDEPENDENT,
    IPLTransmissionType_IPL_TRANSMISSIONTYPE_FREQINDEPENDENT,
};

#[derive(Clone, Copy)]
pub struct DirectEffectFlags {
    apply_distance_attenuation: bool,
    apply_air_absorption: bool,
    apply_directivity: bool,
    apply_occlusion: bool,
    apply_transmission: bool,
}

impl From<DirectEffectFlags> for IPLDirectEffectFlags {
    fn from(value: DirectEffectFlags) -> Self {
        let mut res: u32 = 0;
        if value.apply_air_absorption {
            res += IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYAIRABSORPTION;
        }
        if value.apply_directivity {
            res += IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYDIRECTIVITY;
        }
        if value.apply_distance_attenuation {
            res += IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYDISTANCEATTENUATION;
        }
        if value.apply_occlusion {
            res += IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYOCCLUSION;
        }
        if value.apply_transmission {
            res += IPLDirectEffectFlags_IPL_DIRECTEFFECTFLAGS_APPLYTRANSMISSION;
        }

        res
    }
}

#[derive(Clone, Copy)]
pub enum TransmissionType {
    None,
    FreqIndependent,
    FreqDependent,
}

impl From<TransmissionType> for IPLTransmissionType {
    fn from(value: TransmissionType) -> Self {
        match value {
            TransmissionType::FreqDependent => {
                IPLTransmissionType_IPL_TRANSMISSIONTYPE_FREQDEPENDENT
            }
            _ => IPLTransmissionType_IPL_TRANSMISSIONTYPE_FREQINDEPENDENT,
        }
    }
}

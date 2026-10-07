#![allow(non_upper_case_globals)]

use crate::steamaudio::bind::{
    IPLerror, IPLerror_IPL_STATUS_FAILURE, IPLerror_IPL_STATUS_INITIALIZATION,
    IPLerror_IPL_STATUS_OUTOFMEMORY, IPLerror_IPL_STATUS_SUCCESS,
};

pub enum SteamAudioError {
    Failure,
    OutOfMemory,
    Initialization,
}

pub enum Status {
    Success,
    Unknown(u32),
    SteamAudio(SteamAudioError),
}

impl Status {
    pub fn catch<T>(data: T, status: Status) -> Result<T, Status> {
        if matches!(status, Status::Success) {
            return Ok(data);
        }
        Err(status)
    }
}

impl From<IPLerror> for Status {
    fn from(value: IPLerror) -> Self {
        match value {
            IPLerror_IPL_STATUS_SUCCESS => Self::Success,
            IPLerror_IPL_STATUS_FAILURE => Self::SteamAudio(SteamAudioError::Failure),
            IPLerror_IPL_STATUS_INITIALIZATION => Self::SteamAudio(SteamAudioError::Initialization),
            IPLerror_IPL_STATUS_OUTOFMEMORY => Self::SteamAudio(SteamAudioError::OutOfMemory),
            _ => Self::Unknown(value),
        }
    }
}

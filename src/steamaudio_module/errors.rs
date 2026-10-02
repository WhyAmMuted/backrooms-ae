use super::*;

#[derive(Debug)]
pub enum SteamAudioErrors {
    Success,
    Failure,
    OutOfMemory,
    Initialization,
    Unknown(u32),
}

impl SteamAudioErrors {
    pub fn convert(status: IPLerror) -> SteamAudioErrors {
        match status {
            0 => Self::Success,
            1 => Self::Failure,
            2 => Self::OutOfMemory,
            3 => Self::Initialization,
            _ => Self::Unknown(status),
        }
    }
    pub fn num_error(&self) -> IPLerror {
        match self {
            Self::Failure => IPLerror_IPL_STATUS_FAILURE,
            Self::Success => IPLerror_IPL_STATUS_SUCCESS,
            Self::OutOfMemory => IPLerror_IPL_STATUS_OUTOFMEMORY,
            Self::Initialization => IPLerror_IPL_STATUS_INITIALIZATION,
            Self::Unknown(code) => *code,
        }
    }
}

pub fn catch_error<T>(status: SteamAudioErrors, data: T) -> Result<T, SteamAudioErrors> {
    println!("{}", status.num_error());
    if let SteamAudioErrors::Success = status {
        return Ok(data);
    }
    Err(status)
}

use super::*;

pub fn catch_error<T>(status: IPLerror, data: T) -> Result<T, IPLerror> {
    if status != IPLerror_IPL_STATUS_SUCCESS {
        return Err(status);
    }
    Ok(data)
}

use crate::steamaudio_module::{
    IPLSIMDLevel, IPLSIMDLevel_IPL_SIMDLEVEL_AVX, IPLSIMDLevel_IPL_SIMDLEVEL_AVX2,
    IPLSIMDLevel_IPL_SIMDLEVEL_AVX512, IPLSIMDLevel_IPL_SIMDLEVEL_NEON,
    IPLSIMDLevel_IPL_SIMDLEVEL_SSE2, IPLSIMDLevel_IPL_SIMDLEVEL_SSE4,
};

#[derive(Debug, Clone, Copy)]
pub enum SimdLevels {
    AVX,
    AVX2,
    AVX512,
    NEON,
    SSE2,
    SSE4,
}

impl SimdLevels {
    pub fn convert(&self) -> IPLSIMDLevel {
        match self {
            Self::AVX => IPLSIMDLevel_IPL_SIMDLEVEL_AVX,
            Self::AVX2 => IPLSIMDLevel_IPL_SIMDLEVEL_AVX2,
            Self::AVX512 => IPLSIMDLevel_IPL_SIMDLEVEL_AVX512,
            Self::NEON => IPLSIMDLevel_IPL_SIMDLEVEL_NEON,
            Self::SSE2 => IPLSIMDLevel_IPL_SIMDLEVEL_SSE2,
            Self::SSE4 => IPLSIMDLevel_IPL_SIMDLEVEL_SSE4,
        }
    }
}

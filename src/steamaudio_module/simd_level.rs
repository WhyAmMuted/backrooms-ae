use crate::steamaudio_module::IPLSIMDLevel;

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
            Self::AVX => 2,
            Self::AVX2 => 3,
            Self::AVX512 => 4,
            Self::NEON => 0,
            Self::SSE2 => 0,
            Self::SSE4 => 1,
        }
    }
}

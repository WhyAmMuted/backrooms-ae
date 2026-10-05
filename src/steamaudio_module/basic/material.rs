use crate::steamaudio_module::IPLMaterial;

#[derive(Clone, Copy)]
pub struct Absorption {
    pub low: f32,
    pub middle: f32,
    pub high: f32,
}

impl Default for Absorption {
    fn default() -> Self {
        Absorption {
            low: 0.5,
            middle: 0.5,
            high: 0.5,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Transmission {
    pub low: f32,
    pub middle: f32,
    pub high: f32,
}

impl Default for Transmission {
    fn default() -> Self {
        Transmission {
            low: 0.5,
            middle: 0.5,
            high: 0.5,
        }
    }
}

pub struct Material {
    absorption: Absorption,
    scattering: f32,
    transmission: Transmission,
}

impl Material {
    pub fn new(absortion: Absorption, transmission: Transmission, scattering: f32) -> Self {
        Material {
            absorption: absortion,
            transmission: transmission,
            scattering: scattering,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            absorption: Absorption::default(),
            transmission: Transmission::default(),
            scattering: 0.5,
        }
    }
}

impl From<Material> for IPLMaterial {
    fn from(value: Material) -> Self {
        let absortion = [
            value.absorption.low,
            value.absorption.middle,
            value.absorption.high,
        ];
        let transmission = [
            value.transmission.low,
            value.transmission.middle,
            value.transmission.high,
        ];

        Self {
            absorption: absortion,
            scattering: value.scattering,
            transmission: transmission,
        }
    }
}

//
// From bindings.rs
//
// #[doc = " The acoustic properties of a surface.\n\nYou can specify the acoustic material properties of each triangle, although typically many triangles will\nshare a common material.\n\nThe acoustic material properties are specified for three frequency bands with center frequencies of\n400 Hz, 2.5 KHz, and 15 KHz.\n\nBelow are the acoustic material properties for a few standard materials.\n\n```cpp\n{\"generic\",{0.10f,0.20f,0.30f,0.05f,0.100f,0.050f,0.030f}}\n{\"brick\",{0.03f,0.04f,0.07f,0.05f,0.015f,0.015f,0.015f}}\n{\"concrete\",{0.05f,0.07f,0.08f,0.05f,0.015f,0.002f,0.001f}}\n{\"ceramic\",{0.01f,0.02f,0.02f,0.05f,0.060f,0.044f,0.011f}}\n{\"gravel\",{0.60f,0.70f,0.80f,0.05f,0.031f,0.012f,0.008f}},\n{\"carpet\",{0.24f,0.69f,0.73f,0.05f,0.020f,0.005f,0.003f}}\n{\"glass\",{0.06f,0.03f,0.02f,0.05f,0.060f,0.044f,0.011f}}\n{\"plaster\",{0.12f,0.06f,0.04f,0.05f,0.056f,0.056f,0.004f}}\n{\"wood\",{0.11f,0.07f,0.06f,0.05f,0.070f,0.014f,0.005f}}\n{\"metal\",{0.20f,0.07f,0.06f,0.05f,0.200f,0.025f,0.010f}}\n{\"rock\",{0.13f,0.20f,0.24f,0.05f,0.015f,0.002f,0.001f}}\n```"]
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]

pub enum MaterialPresets {
    Example__,
}

impl From<MaterialPresets> for Material {
    fn from(value: MaterialPresets) -> Self {
        match value {
            MaterialPresets::Example__ => Self {
                absorption: Absorption::default(),
                transmission: Transmission::default(),
                scattering: 0.5,
            },
        }
    }
}

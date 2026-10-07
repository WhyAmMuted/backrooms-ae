#[derive(Clone, Copy)]
pub struct Data {
    pub low: f32,
    pub middle: f32,
    pub high: f32,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            low: 0.02,
            middle: 0.02,
            high: 0.02,
        }
    }
}

pub type Absorption = Data;
pub type Transmission = Data;

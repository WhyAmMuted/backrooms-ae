use crate::steamaudio::bind::IPLAudioSettings;

pub struct AudioSettings {
    sample_rate: i32,
    frame_size: i32,
}

impl AudioSettings {
    pub fn new(sample_rate: i32, frame_size: i32) -> Self {
        Self {
            sample_rate,
            frame_size,
        }
    }

    pub fn as_raw(&self) -> IPLAudioSettings {
        IPLAudioSettings {
            samplingRate: self.sample_rate,
            frameSize: self.frame_size,
        }
    }
}

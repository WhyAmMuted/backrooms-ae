use crate::steamaudio_module::IPLAudioSettings;

pub struct AudioSettings {
    sampling_rate: i32,
    frame_size: i32,
}

impl AudioSettings {
    pub fn new(sampling_rate: i32, frame_size: i32) -> AudioSettings {
        AudioSettings {
            sampling_rate: sampling_rate,
            frame_size: frame_size,
        }
    }

    pub fn get_audio_settings(&self) -> IPLAudioSettings {
        IPLAudioSettings {
            samplingRate: self.sampling_rate,
            frameSize: self.frame_size,
        }
    }

    pub fn sampling_rate(&self) -> i32 {
        self.sampling_rate
    }

    pub fn frame_size(&self) -> i32 {
        self.frame_size
    }
}

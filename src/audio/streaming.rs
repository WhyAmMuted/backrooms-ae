use crate::audio::{
    AudioFileType,
    input::{AudioSource, open_audio},
};
use std::{collections::VecDeque, path::Path};

pub struct AudioStream {
    src: AudioSource,
    sample_queue: VecDeque<f32>,
}

impl AudioStream {
    pub fn open(
        filetype: AudioFileType,
        path: impl AsRef<Path>,
    ) -> Result<AudioStream, Box<dyn std::error::Error>> {
        let src = open_audio(filetype, path)?;
        Ok(AudioStream {
            src,
            sample_queue: VecDeque::new(),
        })
    }

    pub fn next_chunk(&mut self) -> Result<Option<[f32; 2048]>, Box<dyn std::error::Error>> {
        while self.sample_queue.len() < 2048 {
            let packet = match self.src.reader.next_packet() {
                Ok(Some(packet)) => packet,
                Err(symphonia::core::errors::Error::IoError(e))
                    if e.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    break;
                }
                Err(e) => return Err(Box::new(e)),
                Ok(None) => {
                    break;
                }
            };

            if packet.track_id != self.src.track_id {
                continue;
            }

            let decoded = self.src.decoder.decode(&packet)?;

            let mut temp_interleaved: Vec<f32> = Vec::new();

            decoded.copy_to_vec_interleaved(&mut temp_interleaved);

            self.sample_queue.extend(temp_interleaved);
        }

        if self.sample_queue.len() >= 2048 {
            let mut chunk = [0.0f32; 2048];
            for i in 0..2048 {
                chunk[i] = self.sample_queue.pop_front().unwrap();
            }
            Ok(Some(chunk))
        } else if !self.sample_queue.is_empty() {
            let mut chunk = [0.0f32; 2048];
            for i in 0..self.sample_queue.len() {
                chunk[i] = self.sample_queue.pop_front().unwrap();
            }
            Ok(Some(chunk))
        } else {
            Ok(None)
        }
    }
    pub fn get_audio_source(&self) -> &AudioSource {
        &self.src
    }
}

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use cpal::{
    Device, Host, OutputCallbackInfo, Stream, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

pub struct AudioPlay {
    samples: Arc<Mutex<VecDeque<f32>>>,
    host: Host,
    device: Device,
    _stream: Option<Stream>,
    cfg: StreamConfig,
}

impl AudioPlay {
    pub fn new(
        another_device: Option<Device>,
        stream_config: StreamConfig,
    ) -> Result<AudioPlay, u8> {
        let host = cpal::default_host();
        let device = match host.default_output_device() {
            None => {
                if another_device.is_none() {
                    return Err(1);
                } else {
                    another_device.unwrap()
                }
            }
            Some(dvc) => dvc,
        };

        Ok(AudioPlay {
            samples: Arc::new(Mutex::new(VecDeque::new())),
            host: host,
            device: device,
            cfg: stream_config,
            _stream: None,
        })
    }

    pub fn add_samples(&mut self, samples: Vec<f32>) {
        (*self.samples.lock().unwrap()).extend(samples);
    }

    pub fn pop_samples(&mut self, count: usize) -> Vec<f32> {
        let mut samples_guard = self.samples.lock().unwrap();
        let mut count = count;
        if samples_guard.len() < count {
            count = samples_guard.len();
        } else if samples_guard.len() == 0 {
            return Vec::new();
        }
        (*samples_guard).drain(..count).collect()
    }

    pub fn play<E>(&mut self, error_callback: E) -> Result<(), cpal::Error>
    where
        E: FnMut(cpal::Error) + Send + 'static,
    {
        let samples_clone = self.samples.clone();

        let data_callback = move |data: &mut [f32], _info: &OutputCallbackInfo| {
            data.fill(0f32);
            let chunks = data.chunks_exact_mut(2);
            let mut queue = samples_clone.lock().unwrap();
            for i in chunks {
                // i[0] = queue.pop_back().ok_or("Can't get sample").unwrap();
                // i[1] = queue.pop_back().ok_or_else().unwrap();
                i[0] = if let Some(sample) = queue.pop_front() {
                    sample
                } else {
                    0f32
                };
                i[1] = if let Some(sample) = queue.pop_front() {
                    sample
                } else {
                    0f32
                };
            }
        };

        let stream =
            self.device
                .build_output_stream(self.cfg, data_callback, error_callback, None)?;
        stream.play()?;
        self._stream = Some(stream);

        Ok(())
    }
}

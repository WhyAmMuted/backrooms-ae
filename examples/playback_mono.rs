use std::{f32::consts::PI, time::Duration};

use cpal::{
    Error, OutputCallbackInfo, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Audio preparing");

    let sample_rate: u32 = 44100;
    let duration_seconds = 5.0;
    let total_samples: usize = (sample_rate as f32 * duration_seconds) as usize;

    let input_audio: Vec<f32> = (0..total_samples)
        .map(|i| {
            let is_beep = (i % 11025) < 4000;
            if is_beep {
                (2.0 * PI * 880.0 * (i as f32) / (sample_rate as f32)).sin() * 0.5
            } else {
                0.0
            }
        })
        .collect();

    println!(
        "Audio has generated! {} sample rate; {} duration; {} samples",
        sample_rate, duration_seconds, total_samples
    );

    let host = cpal::default_host();
    println!("Host created!");

    let device = host
        .default_output_device()
        .ok_or("Can't get output device")?;
    println!("Device found!");

    let stream_config = StreamConfig {
        channels: 1,
        sample_rate: sample_rate,
        buffer_size: cpal::BufferSize::Default,
    };
    println!("Stream Config created!");

    let mut cur_sample = 0;

    let data_callback = move |data: &mut [f32], _info: &OutputCallbackInfo| {
        data.fill(0f32);
        let chunks = data.chunks_exact_mut(1);

        for i in chunks {
            if let Some(&sample) = input_audio.get(cur_sample) {
                i[0] = sample;
                cur_sample += 1;
            } else {
                break;
            }
        }
    };

    let err_callback = move |err: Error| eprintln!("CPAL Error: {}", err);

    let stream = device.build_output_stream(stream_config, data_callback, err_callback, None)?;

    stream.play()?;

    std::thread::sleep(Duration::from_secs(10));

    Ok(())
}

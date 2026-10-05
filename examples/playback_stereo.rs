use std::{f32::consts::PI, time::Duration};

use backaudio_nexus::realtime_playback::audio::AudioPlay;
use cpal::{Error, StreamConfig};

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

    let mut audio: Vec<f32> = vec![];
    for i in input_audio {
        audio.push(i);
        audio.push(i);
    }

    println!(
        "Audio has generated! {} sample rate; {} duration; {} samples",
        sample_rate, duration_seconds, total_samples
    );
    let stream_config = StreamConfig {
        channels: 2,
        sample_rate: sample_rate,
        buffer_size: cpal::BufferSize::Default,
    };

    let mut audio_play = AudioPlay::new(None, stream_config).expect("Can't create CPAL struct");
    audio_play.add_samples(audio);
    audio_play
        .play(move |err: Error| eprintln!("Error while cpal play: {}", err))
        .expect("Error while cpal play init");

    std::thread::sleep(Duration::from_secs(10));

    Ok(())
}

use std::{ptr::null_mut, sync::Arc, time::Duration};

use BaASteam::{
    audio::streaming::AudioStream,
    realtime_playback::audio::AudioPlay,
    steamaudio_module::{
        basic::{audiobuffer::AudioBuffer, effect::EffectParams},
        *,
    },
};
use cpal::StreamConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello! Steam Audio initialization...");

    let mut stream = AudioStream::open(BaASteam::audio::AudioFileType::Wav, "input.wav")?;

    let sample_rate = stream.get_audio_source().sample_rate;

    println!(
        "Getted {} channels with {} sample rate",
        stream.get_audio_source().channels,
        sample_rate
    );

    let context = Arc::new(create_context(SimdLevels::AVX2).unwrap());
    let mut audio_settings = create_audio_settings(Some(sample_rate as i32), Some(1024));
    let hrtf = create_hrtf(Some(1.0f32), &context, &mut audio_settings).unwrap();
    let effect_l = create_binaural_effect(&context, &mut audio_settings, &hrtf).unwrap();
    let effect_r = create_binaural_effect(&context, &mut audio_settings, &hrtf).unwrap();

    let mut frame_idx = 0;

    let frame_size = audio_settings.frame_size();

    let mut in_buffer = AudioBuffer::new(1, frame_size, context.clone()).unwrap();
    let mut out_buffer = AudioBuffer::new(2, frame_size, context.clone()).unwrap();

    let mut left_mono = vec![0.0f32; frame_size as usize];
    let mut right_mono = vec![0.0f32; frame_size as usize];

    let mut left_stereo = vec![0.0f32; 2 * frame_size as usize];
    let mut right_stereo = vec![0.0f32; 2 * frame_size as usize];

    let mut final_output = vec![0.0f32; 0];

    let mut audio_play = AudioPlay::new(
        None,
        StreamConfig {
            channels: 2,
            sample_rate: sample_rate,
            buffer_size: cpal::BufferSize::Default,
        },
    )
    .expect("Cant CPAL init");
    audio_play.play(move |err| eprintln!("{}", err))?;

    while let Some(chunk) = stream.next_chunk().expect("Cant") {
        //println!("[{}] Progres...", frame_idx);
        for i in 0..frame_size as usize {
            left_mono[i] = chunk[i * 2];
            right_mono[i] = chunk[i * 2 + 1];
        }
        let base_angle = frame_idx as f32 * 0.05;

        let angle_l = base_angle - 0.35;
        let angle_r = base_angle + 0.35;

        in_buffer.deinterleave(&left_mono);

        let params_l = EffectParams {
            direction: Vector3 {
                x: angle_l.cos(),
                y: 0.0,
                z: angle_l.sin(),
            },
            interpolation: basic::hrtf::HRTFInterpolation::Bilinear,
            spatial_blend: 1.0,
            hrtf: &hrtf,
            peak_delays: null_mut(),
        };

        effect_l.apply(&params_l, &mut in_buffer, &mut out_buffer);
        out_buffer.interleave(left_stereo.as_mut_ptr());

        in_buffer.deinterleave(&right_mono);

        let params_r = EffectParams {
            direction: Vector3 {
                x: angle_r.cos(),
                y: 0.0,
                z: angle_r.sin(),
            },
            interpolation: basic::hrtf::HRTFInterpolation::Bilinear,
            spatial_blend: 1.0,
            hrtf: &hrtf,
            peak_delays: null_mut(),
        };

        effect_r.apply(&params_r, &mut in_buffer, &mut out_buffer);
        out_buffer.interleave(right_stereo.as_mut_ptr());

        for i in 0..left_stereo.len() {
            let mixed = (left_stereo[i] + right_stereo[i]) * 0.7;
            final_output.push(mixed);
        }
        audio_play.add_samples(final_output.clone());
        final_output.clear();
        frame_idx += 1;
    }

    println!("Итого обработано фреймов: {}", frame_idx);
    // println!("Processed! Saving..");
    // save_wav("out.wav", &final_output, sample_rate).expect("Cant:(");
    println!("huh..");
    println!("Если воспроизведение закончилось, то нажмите Ctrl+C");

    std::thread::sleep(Duration::from_secs(300));
    Ok(())
}

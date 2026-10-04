use BaASteam::steamaudio_module::{
    basic::{audiobuffer::AudioBuffer, effect::EffectParams},
    *,
};
use std::sync::Arc;
use std::{f32::consts::PI, fs::File, io::Write};

fn main() -> Result<(), SteamAudioErrors> {
    println!("Hello! Steam Audio initialization...");

    let context = Arc::new(create_context(SimdLevels::AVX2)?);
    let mut audio_settings = create_audio_settings(Some(44100), Some(1024));
    let hrtf = create_hrtf(Some(1.0 as f32), &context, &mut audio_settings)?;
    let effect = create_binaural_effect(&context, &mut audio_settings, &hrtf)?;

    println!("Audio preparing");

    let sample_rate: f32 = audio_settings.sampling_rate() as f32;
    let duration_seconds = 5.0;
    let total_samples: usize = (sample_rate * duration_seconds) as usize;

    let input_audio: Vec<f32> = (0..total_samples)
        .map(|i| {
            let is_beep = (i % 11025) < 4000;
            if is_beep {
                (2.0 * PI * 880.0 * (i as f32) / sample_rate).sin() * 0.5
            } else {
                0.0
            }
        })
        .collect();

    println!("Generated {} samples", input_audio.len());
    println!("Buffer allocate");

    let frame_size = audio_settings.frame_size();

    let mut in_buffer = AudioBuffer::new(1, frame_size, context.clone())?;
    let mut out_buffer = AudioBuffer::new(2, frame_size, context.clone())?;

    let mut final_stereo_output: Vec<f32> = Vec::with_capacity(total_samples * 2);
    let mut frame_stereo = vec![0.0f32; 2 * frame_size as usize];
    println!("Main Loop");
    let total_frames = (input_audio.len() / frame_size as usize) as f32;

    for (frame_idx, chunk) in input_audio.chunks(frame_size as usize).enumerate() {
        if chunk.len() < frame_size as usize {
            break;
        }
        in_buffer.deinterleave(chunk);

        let progress = frame_idx as f32 / total_frames as f32;
        let angle = progress * PI * 6.0;

        let effect_param = EffectParams {
            direction: Vector3 {
                x: angle.cos(),
                y: 0.0,
                z: angle.sin(),
            },
            interpolation: basic::hrtf::HRTFInterpolation::Nearest,
            spatial_blend: 1.0,
            hrtf: &hrtf,
            peak_delays: std::ptr::null_mut(),
        };

        effect.apply(&effect_param, &mut in_buffer, &mut out_buffer);
        out_buffer.interleave(frame_stereo.as_mut_ptr());
        final_stereo_output.extend_from_slice(&frame_stereo);
    }
    println!(
        "Wow! Getted {} samples binaural audio",
        final_stereo_output.len()
    );

    save_wav("output.wav", &final_stereo_output, 44100).expect("Failed to save WAV");
    println!("File output.wav saved!");

    Ok(())
}

fn save_wav(filename: &str, samples: &[f32], sample_rate: u32) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let channels: u16 = 2;
    let bits_per_sample: u16 = 16;
    let byte_rate = sample_rate * channels as u32 * (bits_per_sample as u32 / 8);
    let block_align = channels * (bits_per_sample / 8);
    let data_size = samples.len() as u32 * 2;
    let chunk_size = 36 + data_size;

    file.write_all(b"RIFF")?;
    file.write_all(&chunk_size.to_le_bytes())?;
    file.write_all(b"WAVE")?;

    file.write_all(b"fmt ")?;
    file.write_all(&16u32.to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&channels.to_le_bytes())?;
    file.write_all(&sample_rate.to_le_bytes())?;
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&block_align.to_le_bytes())?;
    file.write_all(&bits_per_sample.to_le_bytes())?;

    file.write_all(b"data")?;
    file.write_all(&data_size.to_le_bytes())?;

    for &sample in samples {
        let clamped = sample.clamp(-1.0, 1.0);
        let pcm = (clamped * 32767.0) as i16;
        file.write_all(&pcm.to_le_bytes())?;
    }

    Ok(())
}

// src/audio/input.rs

use std::{fs::File, path::Path};

use symphonia::core::{
    codecs::audio::AudioDecoder,
    formats::{FormatOptions, FormatReader, probe::Hint},
    io::MediaSourceStream,
    meta::MetadataOptions,
};

use crate::audio::AudioFileType;

pub struct AudioSource {
    pub sample_rate: u32,
    pub channels: usize,
    pub track_id: u32,

    pub reader: Box<dyn FormatReader>,
    pub decoder: Box<dyn AudioDecoder>,
}

pub fn open_audio(
    filetype: AudioFileType,
    path: impl AsRef<Path>,
) -> Result<AudioSource, Box<dyn std::error::Error>> {
    println!("Opening file in {:?}", std::fs::canonicalize(&path));
    let file = File::open(path.as_ref())?;
    let source_stream = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    hint.with_extension(filetype.to_str());

    let format = symphonia::default::get_probe().probe(
        &hint,
        source_stream,
        FormatOptions::default(),
        MetadataOptions::default(),
    )?;

    let track = format
        .default_track(symphonia::core::formats::TrackType::Audio)
        .ok_or("Audiotrack not found in file")?;

    let track_id = track.id;
    let codec_params = track
        .codec_params
        .as_ref()
        .and_then(|p| p.audio())
        .ok_or("No audio codec params")?;
    let sample_rate = codec_params.sample_rate.ok_or("Cant detect sample rate")?;
    let channels = codec_params
        .channels
        .as_ref()
        .ok_or("Cant detect channels")?
        .count();

    let decoder =
        symphonia::default::get_codecs().make_audio_decoder(codec_params, &Default::default())?;

    Ok(AudioSource {
        sample_rate,
        channels,
        track_id,
        reader: format,
        decoder: decoder,
    })
}

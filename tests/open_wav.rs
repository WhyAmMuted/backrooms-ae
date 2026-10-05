#[cfg(test)]
mod tests {
    use backaudio_nexus::audio::input::open_audio;

    #[test]
    fn open_wav() {
        let src =
            open_audio(backaudio_nexus::audio::AudioFileType::Wav, "input.wav").expect("Cant open");

        println!("Частота (Sample Rate): {} Гц", src.sample_rate);
        println!("Каналы: {}", src.channels);
        println!("ID дорожки: {}", src.track_id);

        assert!(src.channels > 0);
        assert!(src.sample_rate > 0);
    }
}

#[cfg(test)]
mod tests {
    use BaASteam::audio::streaming::AudioStream;

    #[test]
    fn stream_wav() {
        let mut stream = AudioStream::open(BaASteam::audio::AudioFileType::Wav, "input.wav")
            .expect("Can't open input.wav");

        let mut chunk_count = 0;
        let mut total_samples_read = 0;

        while let Some(chunk) = stream.next_chunk().expect("Error while chunks decode") {
            chunk_count += 1;
            total_samples_read += chunk.len();

            assert!(chunk[0].is_finite(), "Sample NaN or INF");
            assert!(
                chunk[chunk.len() - 1].is_finite(),
                "Sample NaN or INF (last)"
            );

            println!("Всего прочитано чанков: {}", chunk_count);
            println!("Всего прочитано сэмплов f32: {}", total_samples_read);

            // Проверяем, что файл реально прочитался, а не вернул сразу None:
            assert!(chunk_count > 0, "Стрим не прочитал ни одного чанка!");
            assert!(
                total_samples_read >= 2048,
                "Прочитано подозрительно мало сэмплов"
            );

            println!("Тест стриминга успешно пройден!");
        }
    }
}

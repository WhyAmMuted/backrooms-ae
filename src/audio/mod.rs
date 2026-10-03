pub mod input;
pub mod streaming;

pub enum AudioFileType {
    Wav,
    Ogg,
    Mp3,
}

impl AudioFileType {
    pub fn to_str(&self) -> &str {
        match self {
            AudioFileType::Mp3 => "mp3",
            AudioFileType::Ogg => "ogg",
            AudioFileType::Wav => "wav",
        }
    }
}

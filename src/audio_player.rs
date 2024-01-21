use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub struct AudioPlayer {
    sink: Option<Sink>,
    stream_handle: OutputStreamHandle,
}

impl AudioPlayer {
    // Initialize the audio player
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            sink: None,
            stream_handle,
        })
    }

    // Load a WAV file and prepare for playing
    pub fn load_wav_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let buf_reader = BufReader::new(file);
        let source = Decoder::new_wav(buf_reader)?;
        let sink = Sink::try_new(&self.stream_handle)?;
        sink.append(source);
        self.sink = Some(sink);
        Ok(())
    }

    // Play the audio
    pub fn play(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    // Pause the audio
    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }
}

// Your GUI logic here, for example, using egui
// ...

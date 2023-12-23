use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

pub fn play_wav_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    // Get a handle to the default audio output device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    // Open the file
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);

    // Decode the WAV file
    let source = Decoder::new_wav(buf_reader)?;

    // Play the sound
    let sink = rodio::Sink::try_new(&stream_handle)?;
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
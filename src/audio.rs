use rodio::decoder::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub enum AudioCommand {
    SetVolume(f32),
    PlaySong(String),
    Play,
    Pause,
    Queue(String),
    Stop,
    GetState(Sender<AudioState>),
}

#[derive(Clone, Copy)]
pub enum AudioState {
    Playing,
    Paused,
    Stopped,
    NotCreated,
}

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

pub fn create_audio_thread() -> Sender<AudioCommand> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut current_state = AudioState::Stopped; // Initial state
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut sink: Sink = rodio::Sink::try_new(&stream_handle).unwrap();

        for command in receiver {
            match command {
                AudioCommand::Play => {
                    sink.play();
                    current_state = AudioState::Playing;
                }
                AudioCommand::PlaySong(file_path) => {
                    let file = BufReader::new(File::open(file_path).unwrap());
                    let source = Decoder::new_wav(file).unwrap();
                    sink.append(source);
                    current_state = AudioState::Playing;
                }
                AudioCommand::Pause => {
                    sink.pause();
                    current_state = AudioState::Paused;
                }
                AudioCommand::Stop => {
                    sink.stop();
                    current_state = AudioState::Playing;
                } // handle other commands
                AudioCommand::Queue(file_path) => {
                    let file = BufReader::new(File::open(file_path).unwrap());
                    let source = Decoder::new_wav(file).unwrap();
                    sink.append(source);
                    current_state = AudioState::Playing;
                }
                AudioCommand::GetState(sender) => {
                    sender.send(current_state).unwrap(); // Send the current state back
                }
                AudioCommand::SetVolume(volume) => sink.set_volume(volume),
            }
        }
    });

    sender
}

use rodio::decoder::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub enum AudioCommand {
    SetVolume(f32),
    PlaySong(String),
    Play,
    Pause,
    Skip,
    Queue(String),
    Stop,
    GetState(Sender<AudioState>),
}

#[derive(Clone, Copy)]
pub enum AudioState {
    Playing,
    Paused,
    Stopped,
}

struct Queue {
    tracks: Vec<Track>,
}

struct Track {
    file_path: String,
    title: String,
    // artist: String,
    // album: String,
    duration: u32,
    track_progress: u32,
    // track_number: u32,
    // year: u32,
    // genre: String,
}

// Need to implement a current song structure
// If a new track starts playing from the queue we need to update it
// Ways I could implement,

// I could ignore the inbuilt queue from rodio and build my own queue to
// avoid the queues diverging
// This option become more enticing due to the empty function that can
// return true when the sink is empty, prompting a new song to go by grabbing
// from the queue

// I could use the queue since it provides a lot of good features and
// make a failsafe option that always mimics the real queue

pub fn create_audio_thread() -> Sender<AudioCommand> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut current_state = AudioState::Stopped; // Initial state
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink: Sink = rodio::Sink::try_new(&stream_handle).unwrap();

        for command in receiver {
            match command {
                AudioCommand::Play => {
                    sink.play();
                    current_state = AudioState::Playing;
                }
                AudioCommand::PlaySong(file_path) => match file_path.split('.').last() {
                    Some("mp3") => {
                        let file = BufReader::new(File::open(file_path).unwrap());
                        let source = Decoder::new_mp3(file).unwrap();
                        sink.append(source);
                        current_state = AudioState::Playing;
                    }
                    Some("wav") => {
                        let file = BufReader::new(File::open(file_path).unwrap());
                        let source = Decoder::new_wav(file).unwrap();
                        sink.append(source);
                        current_state = AudioState::Playing;
                    }
                    _ => (),
                },
                AudioCommand::Pause => {
                    sink.pause();
                    current_state = AudioState::Paused;
                }
                AudioCommand::Skip => {
                    // This needs to be modiefied in order to make the play
                    // puase functionality work correctly
                    sink.skip_one();
                    current_state = AudioState::Playing;
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

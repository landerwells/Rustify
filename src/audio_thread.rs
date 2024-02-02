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
    GetState(Sender<AudioState>),
}

#[derive(Clone, Copy, Debug)]
pub enum AudioState {
    Playing,
    Paused,
    Stopped,
}

pub fn create_audio_thread() -> Sender<AudioCommand> {
    let (sender, receiver) = mpsc::channel();

    thread::Builder::new()
        .name("Audio Thread".to_string())
        .spawn(move || {
            let mut current_state = AudioState::Stopped; // Initial state
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink: Sink = rodio::Sink::try_new(&stream_handle).unwrap();

            for command in receiver {
                match command {
                    AudioCommand::Play => {
                        sink.play();
                        current_state = AudioState::Playing;
                        println!("The audio state is currently: {:?}", current_state);
                    }
                    AudioCommand::PlaySong(file_path) => {
                        let file = match File::open(&file_path) {
                            Ok(f) => BufReader::new(f),
                            Err(e) => {
                                eprintln!("Error opening file: {}", e);
                                continue; // Skip to the next command if the file can't be opened
                            }
                        };

                        let source_result = match file_path.split('.').last() {
                            Some("mp3") => Decoder::new_mp3(file),
                            Some("wav") => Decoder::new_wav(file),
                            _ => {
                                eprintln!("Unsupported file type");
                                continue; // Skip to the next command for unsupported file types
                            }
                        };

                        match source_result {
                            Ok(source) => {
                                sink.stop();
                                sink.append(source);
                                sink.play();
                                current_state = AudioState::Playing;
                                println!("The audio state is currently: {:?}", current_state);
                            }
                            Err(e) => {
                                eprintln!("Error decoding audio: {}", e);
                                // Handle decoding error, e.g., skip to the next command
                            }
                        }
                    }

                    AudioCommand::Pause => {
                        sink.pause();
                        current_state = AudioState::Paused;
                        println!("The audio state is currently: {:?}", current_state);
                    }
                    AudioCommand::Skip => {
                        if sink.len() <= 1 {
                            sink.stop();
                            current_state = AudioState::Stopped;
                            println!("The audio state is currently: {:?}", current_state);
                        } else {
                            sink.skip_one();
                            current_state = AudioState::Playing;
                            println!("The audio state is currently: {:?}", current_state);
                        }
                    }
                    AudioCommand::GetState(sender) => sender.send(current_state).unwrap(),
                    AudioCommand::SetVolume(volume) => sink.set_volume(volume),
                }
            }
        })
        .unwrap(); // Handle errors as appropriate for your use case

    sender
}

use rodio::decoder::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant};

pub enum AudioCommand {
    SetVolume(f32),
    PlaySong(String),
    SetProgress(f32),
    GetProgress(Sender<Duration>),
    GetTrackDuration(Sender<Duration>),
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
            // let mut source: Option<Decoder<BufReader<File>>> = None;
            let mut start_time: Option<Instant> = None;
            let mut track_duration: Option<Duration> = None;

            for command in receiver {
                match command {
                    AudioCommand::GetTrackDuration(sender) => {
                        let _ = sender.send(track_duration.unwrap());
                    }
                    AudioCommand::GetProgress(sender) => {
                        let elapsed_time = start_time
                            .map(|start| start.elapsed())
                            .unwrap_or_else(|| Duration::ZERO);
                        let _ = sender.send(elapsed_time); // Send the elapsed time back
                    }
                    AudioCommand::SetProgress(progress) => {
                        // Set the progress of the currently playing track
                        // This is where you would update the progress bar in the UI
                        // lets test with the progress bar whether it works.
                        println!("The progress of the track is: {}", progress);
                    }
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

                        // Since we're only dealing with .wav files, directly attempt to decode as wav
                        let source_result = Decoder::new_wav(file);

                        start_time = Some(Instant::now());
                        match source_result {
                            Ok(source) => {
                                sink.stop();
                                track_duration = source.total_duration();
                                sink.append(source);
                                sink.play();
                                current_state = AudioState::Playing;
                                println!("The audio state is currently: {:?}", current_state);
                            }
                            Err(e) => {
                                eprintln!("Error decoding .wav file: {}", e);
                                // Handle decoding error
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

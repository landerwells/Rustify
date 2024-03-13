use rodio::decoder::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use rodio::Source;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

pub enum AudioCommand {
    GetProgress(Sender<Duration>),
    GetState(Sender<AudioState>),
    GetTrackDuration(Sender<Duration>),
    Pause,
    Play,
    PlaySong(String),
    SetProgress(f32, String),
    SetVolume(f32),
    Skip,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum AudioState {
    Playing,
    Paused,
    Empty,
}

pub fn create_audio_thread() -> Sender<AudioCommand> {
    let (sender, receiver) = mpsc::channel();

    thread::Builder::new()
        .name("Audio Thread".to_string())
        .spawn(move || {
            let mut current_state = AudioState::Empty; // Initial state
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink: Sink = rodio::Sink::try_new(&stream_handle).unwrap();
            let mut source: Decoder<BufReader<File>>;
            let mut start_time: Option<Instant> = None;
            let mut track_duration: Option<Duration> = None;
            let mut last_pause_time: Option<Duration> = None;
            let mut elapsed_before_pause = Duration::ZERO; // Track total elapsed time before the last pause


            for command in receiver {
                match command {
                    AudioCommand::GetTrackDuration(sender) => {
                        if current_state == AudioState::Empty {
                            let _ = sender.send(Duration::from_secs(1));
                            continue;
                        }
                        let _ = sender.send(track_duration.unwrap());
                    }
                    AudioCommand::GetProgress(sender) => {
                        // if the time the track has been playing exceeds the 
                        // duration of the track, stop the track
                        if current_state == AudioState::Playing {
                            if let Some(track_duration) = track_duration {
                                if start_time.unwrap().elapsed() > track_duration {
                                    sink.stop();
                                    current_state = AudioState::Empty;
                                }
                            }
                        }
                        let elapsed_time = match current_state {
                            AudioState::Playing => {
                                // If playing, calculate the elapsed time since the start or since the last resume
                                let elapsed_since_start = start_time.unwrap_or_else(Instant::now).elapsed();
                                elapsed_before_pause + elapsed_since_start // Add the time before the last pause
                            },
                            AudioState::Paused | AudioState::Empty => {
                                // When paused or stopped, keep returning the time at the moment of the last pause
                                elapsed_before_pause
                            },
                        };
                        let _ = sender.send(elapsed_time);
                    },
                    AudioCommand::SetProgress(progress, file_path) => {
                        sink.stop();
                        let file = match File::open(&file_path) {
                            Ok(f) => BufReader::new(f),
                            Err(e) => {
                                eprintln!("Error opening file: {}", e);
                                continue;
                            }
                        };
                        source = Decoder::new_wav(file).unwrap();
                        // Set the position where the track should start playing from, based on the progress.
                        let skip_duration = Duration::from_secs_f32(progress);
                        sink.append(source.skip_duration(skip_duration));

                        // Adjust the start_time to correctly reflect the new starting point.
                        // If we're currently playing, we should resume playing from the new position.
                        // If we're paused, we adjust the timing but do not start playback.
                        if current_state == AudioState::Playing {
                            sink.play();
                            start_time = Some(Instant::now() - skip_duration);
                        } else if current_state == AudioState::Paused {
                            // Adjust start_time to reflect the progress point without resuming playback.
                            // This ensures that when we resume, we do so from the correct point.
                            start_time = Some(Instant::now() - skip_duration);
                            // Since we're paused, we update elapsed_before_pause to reflect the current progress.
                            elapsed_before_pause = skip_duration;
                        }
                    },
                    AudioCommand::Play => {
                        if current_state != AudioState::Playing {
                            sink.play();
                            current_state = AudioState::Playing;
                            start_time = Some(Instant::now()); // Reset start_time on play if not already playing
                        }
                        println!("The audio state is currently: {:?}", current_state);
                    },
                        AudioCommand::PlaySong(file_path) => {
                            let file = match File::open(&file_path) {
                                Ok(f) => BufReader::new(f),
                                Err(e) => {
                                    eprintln!("Error opening file: {}", e);
                                    continue;
                                }
                            };
                            println!("{}", file_path);

                            // Since we're only dealing with .wav files, directly attempt to decode as wav
                            source = Decoder::new_wav(file).unwrap();
                            track_duration = Some(source.total_duration().unwrap());
                            sink.stop();
                            sleep(Duration::from_millis(200));
                            sink.append(source);
                            sink.play();
                            current_state = AudioState::Playing;
                            start_time = Some(Instant::now());
                            // Reset elapsed_before_pause to 0 when starting a new track
                            elapsed_before_pause = Duration::ZERO;
                                 
                            println!("The audio state is currently: {:?}", current_state);
                        }
                    AudioCommand::Pause => {
                        sink.pause();
                        current_state = AudioState::Paused;
                        // Update elapsed time at the moment of pause
                        elapsed_before_pause += start_time.unwrap_or_else(Instant::now).elapsed();
                        println!("The audio state is currently: {:?}", current_state);
                    },
                    AudioCommand::Skip => {
                        if sink.len() <= 1 {
                            sink.stop();
                            current_state = AudioState::Empty;
                            println!("The audio state is currently: {:?}", current_state);
                        } else {
                            sink.skip_one();
                            current_state = AudioState::Playing;
                            println!("The audio state is currently: {:?}", current_state);
                        }
                    }
                    AudioCommand::GetState(sender) => {
                        if sink.empty() {
                            current_state = AudioState::Empty;
                        }
                        sender.send(current_state).unwrap()
                    }
                    AudioCommand::SetVolume(volume) => sink.set_volume(volume),
                }
            }
        })
    .unwrap();

    sender
}

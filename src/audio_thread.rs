use rodio::decoder::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use rodio::Source;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread;
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
            let mut last_pause_time: Option<Instant> = None;
            let mut track_duration: Option<Duration> = None;

            for command in receiver {
                match command {

                    AudioCommand::GetProgress(sender) => {
                        // Debugging:
                        // println!("Current state: {:?}", current_state);
                        let progress = match current_state {
                            AudioState::Empty => {
                                Duration::from_secs(0)
                            },
                            AudioState::Paused => {
                                // If paused, calculate progress up to the pause time
                                last_pause_time.map_or(Duration::from_secs(0), |pause_time| {
                                    start_time.map_or(Duration::from_secs(0), |start| pause_time.duration_since(start))
                                })
                            },
                            AudioState::Playing => {
                                start_time.map_or(Duration::from_secs(0), |start| start.elapsed())
                            },
                        };

                        // Debugging:
                        // println!("Progress: {:?}", progress);
                        let _ = sender.send(progress);

                        // Reset last_pause_time to None when playing
                        if current_state == AudioState::Playing {
                            last_pause_time = None;
                        }
                    },

                    // this method might need some more work to be production ready
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
                        }
                    },

                    AudioCommand::Play => {
                        // Calculate the total paused duration if there's a last_pause_time.
                        // This assumes `last_pause_time` represents the moment the latest pause began,
                        // and `elapsed_before_pause` accumulates all pause durations before the latest one.
                        if let Some(last_pause) = last_pause_time {
                            let pause_duration = Instant::now().duration_since(last_pause);
                            // Adjust the start_time by the total pause duration (including the most recent pause).
                            // This essentially "moves" the start_time forward by pause_duration, shortening the total elapsed time by the paused duration.
                            start_time = Some(start_time.unwrap_or(Instant::now()) + pause_duration);
                        } else {
                            // If there's no pause (e.g., starting playback for the first time), initialize start_time to now.
                            start_time = Some(Instant::now());
                        }

                        // Reset last_pause_time as playback is resuming.
                        last_pause_time = None;

                        // Start or resume playback.
                        sink.play();

                        // Update current state to Playing.
                        current_state = AudioState::Playing;
                    },

                    AudioCommand::PlaySong(file_path) => {
                        // extract out into seperate method
                        let file = match File::open(&file_path) {
                            Ok(f) => BufReader::new(f),
                            Err(e) => {
                                eprintln!("Error opening file: {}", e);
                                continue;
                            }
                        };

                        source = Decoder::new_wav(file).unwrap();
                        track_duration = Some(source.total_duration().unwrap());
                        sink.clear();
                        sink.append(source);
                        sink.play();
                        start_time = Some(Instant::now());
                        last_pause_time = None; 
                        current_state = AudioState::Playing;
                    }

                    AudioCommand::Pause => {
                        sink.pause();
                        current_state = AudioState::Paused;
                        last_pause_time = Some(Instant::now());
                        println!("The audio state is currently: {:?}", current_state);
                    },
                    // Below this nothing should need to be changed
                    AudioCommand::Skip => {
                        if sink.len() <= 1 {
                            sink.stop();
                            current_state = AudioState::Empty;
                        } else {
                            sink.skip_one();
                            current_state = AudioState::Playing;
                        }
                        println!("The audio state is currently: {:?}", current_state);
                    }
                    AudioCommand::GetTrackDuration(sender) => {
                        if current_state == AudioState::Empty {
                            let _ = sender.send(Duration::from_secs(1));
                            continue;
                        }
                        let _ = sender.send(track_duration.unwrap());
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

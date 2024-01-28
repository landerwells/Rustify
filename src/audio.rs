use rodio::decoder::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

pub enum AudioCommand {
    SetVolume(f32),
    PlaySong(String),
    Play,
    Pause,
    Skip,
    // GetTrackDuration,
    Queue(String),
    Stop,
    GetState(Sender<AudioState>),
}

#[derive(Clone, Copy, Debug)]
pub enum AudioState {
    Playing,
    Paused,
    Stopped,
}

struct Queue {
    tracks: Vec<Track>,
}

struct Track {
    // album: String,
    // artist: String,
    // genre: String,
    // track_number: u32,
    // type duration
    // year: u32,
    duration: Option<Duration>,
    file_path: String,
    source: Decoder<BufReader<File>>,
    track_progress: Option<Duration>,
}

impl Track {
    pub fn new(file_path: String) -> Result<Track, String> {
        let file = BufReader::new(File::open(&file_path).map_err(|e| e.to_string())?);

        let source = match file_path.split('.').last() {
            Some("mp3") => Decoder::new_mp3(file).map_err(|e| e.to_string())?,
            Some("wav") => Decoder::new_wav(file).map_err(|e| e.to_string())?,
            _ => return Err("Unsupported file format".to_string()),
        };

        let duration = source.total_duration();

        Ok(Track {
            file_path,
            source,
            duration,
            track_progress: Some(Duration::from_secs(0)),
        })
    }
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

// I could look into rodio::queue and see if that solves any of my issues

pub fn create_audio_thread() -> Sender<AudioCommand> {
    let (sender, receiver) = mpsc::channel();

    thread::Builder::new()
        .name("Audio Thread".to_string())
        .spawn(move || {
            let mut current_state = AudioState::Stopped; // Initial state
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink: Sink = rodio::Sink::try_new(&stream_handle).unwrap();

            // let mut queue = Queue { tracks: Vec::new() };

            for command in receiver {
                match command {
                    AudioCommand::Play => {
                        sink.play();
                        current_state = AudioState::Playing;
                        println!("The audio state is currently: {:?}", current_state);
                    }
                    AudioCommand::PlaySong(file_path) => {
                        let song = Track::new(file_path);
                        sink.stop();
                        sink.append(song.unwrap().source);
                        // queue.tracks.insert(0, song.unwrap());
                        sink.play();
                        current_state = AudioState::Playing;
                        println!("The audio state is currently: {:?}", current_state);
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
                    AudioCommand::Stop => {
                        sink.stop();
                        current_state = AudioState::Playing;
                        println!("The audio state is currently: {:?}", current_state);
                    }
                    AudioCommand::Queue(file_path) => {
                        let file = BufReader::new(File::open(file_path).unwrap());
                        let source = Decoder::new_wav(file).unwrap();
                        sink.append(source);
                        current_state = AudioState::Playing;
                    }
                    AudioCommand::GetState(sender) => sender.send(current_state).unwrap(),
                    AudioCommand::SetVolume(volume) => sink.set_volume(volume),
                    // AudioCommand::GetTrackDuration => {
                    //     queue.tracks[0].duration.unwrap();
                    // }
                }
            }
        })
        .unwrap(); // Handle errors as appropriate for your use case

    sender
}

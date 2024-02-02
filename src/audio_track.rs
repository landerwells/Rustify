use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub title: String,
    pub file_path: String,
    duration: Option<Duration>,
    track_progress: Option<Duration>,
}

impl Track {
    pub fn new(file_path: String) -> Result<Track, String> {
        let title = file_path.split('/').last().unwrap().to_string();

        Ok(Track {
            title,
            file_path,
            duration: Some(Duration::from_secs(0)),
            track_progress: Some(Duration::from_secs(0)),
        })
    }
}

pub fn get_tracks() -> Vec<Track> {
    let mut tracks = Vec::new();
    if let Ok(entries) = fs::read_dir("assets/tracks") {
        for entry in entries.filter_map(Result::ok) {
            let file_path: String = entry.path().to_str().unwrap().to_string();
            let track = Track::new(file_path).unwrap();
            tracks.push(track);
        }
    }
    tracks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_new() {
        let file_path = "path/to/song.mp3".to_string();
        let track = Track::new(file_path.clone()).unwrap();

        assert_eq!(track.file_path, file_path);
        assert_eq!(track.title, "song.mp3");
        assert!(track.duration.is_some());
        assert_eq!(track.duration.unwrap(), Duration::from_secs(0));
        assert!(track.track_progress.is_some());
        assert_eq!(track.track_progress.unwrap(), Duration::from_secs(0));
    }
}

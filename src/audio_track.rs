use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub title: String,
    pub file_path: String,
    duration: Option<Duration>,
    track_progress: Option<Duration>,
}

impl Track {
    pub fn new(file_path: String) -> Result<Track, String> {
        let mut title = file_path.split('/').last().unwrap().to_string();
        title = title.split('.').next().unwrap().to_string();

        Ok(Track {
            title,
            file_path,
            duration: Some(Duration::from_secs(0)),
            track_progress: Some(Duration::from_secs(0)),
        })
    }
}

// #[derive(Serialize, Deserialize)]
// pub struct TrackList {
//     pub tracks: Vec<Track>,
// }

// impl TrackList {
// pub fn new() -> Self {
//     Self { tracks: Vec::new() }
// }

//     pub fn add_track(&mut self, track: Track) {
//         self.tracks.push(track);
//     }

//     pub fn remove_track(&mut self, track: Track) {
//         self.tracks.retain(|t| t.file_path != track.file_path);
//     }
// }

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

// impl IntoIterator for TrackList {
//     type Item = Track;
//     type IntoIter = IntoIter<Track>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.tracks.into_iter()
//     }
// }

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

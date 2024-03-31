use crate::audio_track::Track;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tracks: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn remove_track(&mut self, track: Track) {
        self.tracks.retain(|t| t.file_path != track.file_path);
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn delete(&mut self) {
        self.tracks.clear();
    }
}

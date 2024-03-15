use crate::audio_track::Track;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Queue {
    pub tracks: Vec<Track>,
}

impl Queue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_track(&mut self, track: Track) {
        let title = track.title.clone();
        self.tracks.push(track);
        println!("The track {} has been added to the queue", title);
    }

    pub fn remove_track(&mut self, track: Track) {
        self.tracks.retain(|t| t.file_path != track.file_path);
    }

    pub fn get_tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }
}

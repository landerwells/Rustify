// need to be able to create a new playlist
// rename playlist
// delete playlist
// add song to playlist
// remove song from playlist
// play playlist
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
use crate::audio_track::Track;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlaylistList {
    pub playlists: Vec<Playlist>,
}

impl PlaylistList {
    pub fn new() -> Self {
        Self {
            playlists: Vec::new(),
        }
    }

    pub fn add_playlist(&mut self, playlist: Playlist) {
        self.playlists.push(playlist);
    }

    pub fn remove_playlist(&mut self, playlist: Playlist) {
        self.playlists.retain(|p| p.name != playlist.name);
    }
}

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

    pub fn play(&self) {
        // play the playlist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}

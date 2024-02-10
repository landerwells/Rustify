// Queue should play tracks in order, and

// struct Queue {
//     tracks: Vec<Track>,
// }

// When we click play on a song, it should start to play it

//

// There should be different priority levels for playing tracks

// Highest priority is when you click on a track to play it

// Next any songs that you have queued up to play

// Finally if you were playing any playlist, it should play the next song in the playlist
// until the playlist is over

// If there is nothing else to play, stop the sink.

use crate::audio_track::Track;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Queue {
    // needs to implement std::marker::Copy
    pub tracks: Vec<Track>,
}

impl Queue {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    pub fn add_track(&mut self, track: Track) {
        let title = track.title.clone();
        self.tracks.push(track);
        println!("The track {} has been added to the queue", title);
    }

    pub fn remove_track(&mut self, track: Track) {
        self.tracks.retain(|t| t.file_path != track.file_path);
    }

    pub fn play(&self) {
        // play the queue
    }
}

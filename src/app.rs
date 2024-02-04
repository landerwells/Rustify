use crate::audio_thread::create_audio_thread;
use crate::audio_thread::AudioCommand;
use crate::audio_thread::AudioState;
use crate::audio_track::get_tracks;
use crate::audio_track::Track;
use crate::playlist::Playlist;
use crate::playlist::PlaylistList;
use crate::ui;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    pub is_playing: bool,
    pub volume: f32,
    pub track_progress: f32,
    // playlist_list: PlaylistList,
    pub playlist_list: Vec<Playlist>,

    #[serde(skip)]
    pub audio_thread_sender: std::sync::mpsc::Sender<AudioCommand>,

    #[serde(skip)]
    pub track_list: Vec<Track>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            is_playing: false,
            volume: 1.0,
            track_progress: 0.0,
            track_list: get_tracks(),
            audio_thread_sender: create_audio_thread(),
            // playlist_list: PlaylistList::new(),
            playlist_list: Vec::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        let (state_sender, state_receiver) = std::sync::mpsc::channel();
        self.audio_thread_sender
            .send(AudioCommand::GetState(state_sender))
            .unwrap();

        // Receive the current state
        match state_receiver.recv() {
            Ok(AudioState::Playing) => {
                self.is_playing = true;
            }
            Ok(AudioState::Paused) => {
                self.is_playing = false;
            }
            Ok(AudioState::Stopped) => {
                self.is_playing = false;
            }
            _ => (),
        }

        // Request current song progress
        let (sender, progress_receiver) = std::sync::mpsc::channel();
        self.audio_thread_sender
            .send(AudioCommand::GetProgress(sender))
            .unwrap();

        let (sender, duration_receiver) = std::sync::mpsc::channel();
        self.audio_thread_sender
            .send(AudioCommand::GetProgress(sender))
            .unwrap();

        if let Ok(progress) = progress_receiver.recv() {
            if let Ok(duration) = duration_receiver.recv() {
                self.track_progress = progress.as_secs_f32() / duration.as_secs_f32();
            }
        }

        // Top Panel:
        // Responsible for displaying the menu bar and the dark/light mode
        // buttons.
        ui::top_panel::show_top_panel(ctx, _frame, self);

        // Side Panel:
        // Responisble for displaying all the playlists and the ability to add
        // new ones.
        ui::side_panel::show_side_panel(ctx, self);

        // Central Panel:
        // Responsible for displaying all tracks or tracks in the current
        // playlist.
        ui::central_panel::show_central_panel(ctx, self);

        // Bottom Panel:
        // Responsible for displaying the volume slider, play/pause button,
        // skip button, and the track progress bar.
        ui::bottom_panel::show_bottom_panel(ctx, self);
    }
}

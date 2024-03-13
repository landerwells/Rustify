use crate::audio_thread::create_audio_thread;
use crate::audio_thread::AudioCommand;
use crate::audio_thread::AudioState;
use crate::audio_track;
use crate::audio_track::Track;
use crate::playlist::Playlist;
use crate::queue::Queue;
use crate::ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    pub audio_state: AudioState,
    pub volume: f32,
    pub track_progress: f32,
    pub track_duration: f32,
    pub queue: Queue,
    pub show_playlist_input: bool,
    pub new_playlist_name: String,
    pub playlist_creation_error: Option<String>,
    pub playlist_list: Vec<Playlist>,

    #[serde(skip)]
    pub audio_thread_sender: std::sync::mpsc::Sender<AudioCommand>,
    #[serde(skip)]
    pub current_playlist: Option<String>,
    #[serde(skip)]
    pub track_list: Vec<Track>,
    #[serde(skip)]
    pub current_track: Option<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            audio_state: AudioState::Empty,
            audio_thread_sender: create_audio_thread(),
            current_track: None,
            new_playlist_name: String::new(),
            playlist_creation_error: None,
            playlist_list: Vec::new(),
            queue: Queue::new(),
            show_playlist_input: false,
            track_duration: 0.0,
            track_list: audio_track::get_tracks(),
            track_progress: 0.0,
            volume: 1.0,
            current_playlist: None,
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

    // Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Panel:
        // Responsible for displaying the menu bar and the dark/light mode
        // buttons.
        ui::top_panel::show_top_panel(ctx, _frame, self);

        // Side Panel:
        // Responsible for displaying all the playlists and the ability to add
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

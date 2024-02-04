use crate::audio_thread::create_audio_thread;
use crate::audio_thread::AudioCommand;
use crate::audio_thread::AudioState;
use crate::audio_track::get_tracks;
use crate::audio_track::Track;
use crate::playlist::Playlist;
use crate::playlist::PlaylistList;
use egui::widgets::Label;
use egui::Sense;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    is_playing: bool,
    volume: f32,
    track_progress: f32,
    track_list: Vec<Track>,
    // playlist_list: PlaylistList,
    playlist_list: Vec<Playlist>,

    #[serde(skip)]
    audio_thread_sender: std::sync::mpsc::Sender<AudioCommand>,
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        // Playlists
        // Getting some coupling here, would like to extract the more technical
        // details into the playlist module but not for right now.
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Playlists");
            // Add a new playlist
            if ui.button("Add Playlist").clicked() {
                let mut text = String::new();
                ui.text_edit_singleline(&mut text);
                print!("Playlist name: {}", text);
                // new playlist should be named by the user
                // pop up with text field and take the name

                //
                let playlist = Playlist::new("New Playlist".to_string());
                self.playlist_list.push(playlist);
            }
            ui.separator();

            let mut playlists_to_delete: Vec<String> = Vec::new();

            for playlist in &self.playlist_list {
                let button = ui.button(&playlist.name);

                if button.clicked() {
                    println!("Playlist: {}", playlist.name);
                    // Left-click logic
                    // e.g., Display songs in this playlist in the central display
                }

                button.context_menu(|ui| {
                    if ui.button("Delete Playlist").clicked() {
                        playlists_to_delete.push(playlist.name.clone());
                    }
                });
            }
            self.playlist_list
                .retain(|p| !playlists_to_delete.contains(&p.name));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Track List");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for track in &self.track_list {
                        if ui.button(&track.title).clicked() {
                            // Logic to play the track
                            // Example: send a play command with the track's file path
                            self.audio_thread_sender
                                .send(AudioCommand::PlaySong(track.file_path.clone()))
                                .unwrap();
                        }
                        ui.separator();
                    }
                });
            });
        });

        // The central panel the region left after adding TopPanel's and SidePanel's
        // ui.heading("Rustify");

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // Volume slider
                // let mut volume = self.volume; // Assuming `self.volume` holds the current volume
                ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).text("Volume"));
                self.audio_thread_sender
                    .send(AudioCommand::SetVolume(self.volume))
                    .unwrap();

                let button_label = if self.is_playing { "⏸" } else { "▶" };
                if ui.button(button_label).clicked() {
                    let (state_sender, state_receiver) = std::sync::mpsc::channel();
                    self.audio_thread_sender
                        .send(AudioCommand::GetState(state_sender))
                        .unwrap();

                    // Receive the current state
                    match state_receiver.recv() {
                        Ok(AudioState::Playing) => {
                            self.audio_thread_sender.send(AudioCommand::Pause).unwrap();
                        }
                        Ok(AudioState::Paused) => {
                            self.audio_thread_sender.send(AudioCommand::Play).unwrap();
                        }
                        _ => (),
                    }
                }
                if ui.button("⏭").clicked() {
                    self.audio_thread_sender.send(AudioCommand::Skip).unwrap();
                }

                if ui
                    .add(
                        egui::Slider::new(&mut self.track_progress, 0.0..=1.0)
                            .text("Track Progress"),
                    )
                    .changed()
                {
                    // This block will only execute if the slider's value has changed
                    self.audio_thread_sender
                        .send(AudioCommand::SetProgress(self.track_progress))
                        .unwrap();
                }
            });
        });
    }
}

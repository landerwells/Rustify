use crate::audio_thread::create_audio_thread;
use crate::audio_thread::AudioCommand;
use crate::audio_thread::AudioState;
use crate::audio_track::get_tracks;
use crate::audio_track::Track;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    is_playing: bool,
    volume: f32,
    track_progress: f32,
    track_list: Vec<Track>,

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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
            ui.label("Lorem ipsum");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Track List");

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
                // if volume != self.volume {
                // self.volume = volume;
                // Send volume update to the audio thread
                self.audio_thread_sender
                    .send(AudioCommand::SetVolume(self.volume))
                    .unwrap();
                // }
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

                // Find current track duration

                // find current track progress
                // divide

                ui.add(
                    egui::Slider::new(&mut self.track_progress, 0.0..=1.0).text("Track Progress"),
                );
            });
        });

        fn nested_menus(ui: &mut egui::Ui) {
            if ui.button("Open...").clicked() {
                ui.close_menu();
            }
            ui.menu_button("SubMenu", |ui| {
                ui.menu_button("SubMenu", |ui| {
                    if ui.button("Open...").clicked() {
                        ui.close_menu();
                    }
                    let _ = ui.button("Item");
                });
                ui.menu_button("SubMenu", |ui| {
                    if ui.button("Open...").clicked() {
                        ui.close_menu();
                    }
                    let _ = ui.button("Item");
                });
                let _ = ui.button("Item");
                if ui.button("Open...").clicked() {
                    ui.close_menu();
                }
            });
            ui.menu_button("SubMenu", |ui| {
                let _ = ui.button("Item1");
                let _ = ui.button("Item2");
                let _ = ui.button("Item3");
                let _ = ui.button("Item4");
                if ui.button("Open...").clicked() {
                    ui.close_menu();
                }
            });
            let _ = ui.button("Very long text for this item");
        }
    }
}

use crate::audio::create_audio_thread;
use crate::audio::play_wav_file;
use crate::audio::AudioCommand;
use crate::audio::AudioState;
use std::fs;
use std::thread;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    volume: f32,
    #[serde(skip)] // This how you opt-out of serialization of a field
    audio_thread_sender: std::sync::mpsc::Sender<AudioCommand>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            volume: 1.0,
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

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::CentralPanel::default().show(ctx, |ui| {
                if let Ok(entries) = fs::read_dir(".") {
                    entries
                        .filter_map(Result::ok)
                        .filter(|entry| {
                            entry.path().is_file()
                                && entry
                                    .path()
                                    .extension()
                                    .map_or(false, |ext| ext == "wav" || ext == "mp3")
                        })
                        .for_each(|entry| {
                            let path = entry.path();
                            if ui.button(path.to_string_lossy()).clicked() {
                                let file_path = path.to_string_lossy().to_string();
                                // Send play command to the audio thread
                                self.audio_thread_sender
                                    .send(AudioCommand::PlaySong(file_path))
                                    .unwrap();
                            }
                        });
                }
            });

            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("Rustify");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.separator();

            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     powered_by_egui_and_eframe(ui);
            //     egui::warn_if_debug_build(ui);
            // });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // Volume slider
                let mut volume = self.volume; // Assuming `self.volume` holds the current volume
                ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).text("Volume"));
                if volume != self.volume {
                    self.volume = volume;
                    // Send volume update to the audio thread
                    self.audio_thread_sender
                        .send(AudioCommand::SetVolume(volume))
                        .unwrap();
                }
                // if ui.button("Play/Pause").clicked() {
                //     if self.audio_thread_sender.send(AudioCommand::GetState) == "Playing" {
                //         self.audio_thread_sender.send(AudioCommand::Pause).unwrap();
                //     }
                //     let file_path = "CantinaBand60.wav"; // Replace with the actual file path
                //     self.audio_thread_sender
                //         .send(AudioCommand::Play("CantinaBand60.wav".to_string()))
                //         .unwrap();

                //     // thread::spawn(|| {
                //     //     if let Err(e) = play_wav_file(file_path) {
                //     //         eprintln!("Error playing file: {}", e);
                //     //     }
                //     // });
                // }
                if ui.button("Play/Pause").clicked() {
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
                        _ => {
                            let file_path = "CantinaBand60.wav".to_string();
                            self.audio_thread_sender
                                .send(AudioCommand::PlaySong(file_path))
                                .unwrap();
                        }
                    }
                }
            });
        });
    }
}

// fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//     ui.horizontal(|ui| {
//         ui.spacing_mut().item_spacing.x = 0.0;
//         ui.label("Powered by ");
//         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//         ui.label(" and ");
//         ui.hyperlink_to(
//             "eframe",
//             "https://github.com/emilk/egui/tree/master/crates/eframe",
//         );
//         ui.label(".");
//     });
// }

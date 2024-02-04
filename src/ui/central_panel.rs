use crate::audio_thread::AudioCommand;
use crate::TemplateApp;

use eframe::egui; // Make sure to import necessary modules // Import your app struct if needed

pub fn show_central_panel(ctx: &egui::Context, app: &mut TemplateApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.heading("Track List");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for track in &app.track_list {
                    if ui.button(&track.title).clicked() {
                        // Logic to play the track
                        // Example: send a play command with the track's file path
                        app.audio_thread_sender
                            .send(AudioCommand::PlaySong(track.file_path.clone()))
                            .unwrap();
                    }
                    ui.separator();
                }
            });
        });
    });
}

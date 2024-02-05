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
                    let button = ui.button(&track.title);

                    if button.clicked() {
                        // Logic to play the track
                        // Example: send a play command with the track's file path
                        app.audio_thread_sender
                            .send(AudioCommand::PlaySong(track.file_path.clone()))
                            .unwrap();
                    }

                    button.context_menu(|ui| {
                        // add to queue
                        if ui.button("Add to Queue").clicked() {
                            // Logic to add the track to the queue
                            // Example: add the track to the queue list
                        }
                        ui.menu_button("Add to Playlist", |ui| {
                            for playlist in &app.playlist_list {
                                if ui.button(&playlist.name).clicked() {
                                    // Logic to add the track to the playlist
                                    // Example: add the track to the playlist's track list
                                }
                            }
                        });
                    });

                    ui.separator();
                }
            });
        });
    });
}

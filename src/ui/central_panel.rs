use crate::audio_thread::AudioCommand;
use crate::TemplateApp;

use eframe::egui; // Make sure to import necessary modules // Import your app struct if needed

pub fn show_central_panel(ctx: &egui::Context, app: &mut TemplateApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            // Heading needs to reflect which playlist is currently being shown
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
                            app.queue.add_track(track.clone());
                            // Find a way to close the context menu since I
                            // don't want it to stay open.
                        }
                        ui.menu_button("Add to Playlist", |ui| {
                            for playlist in &mut app.playlist_list {
                                if ui.button(&playlist.name).clicked() {
                                    // Logic to add the track to the playlist
                                    playlist.add_track(track.clone());
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

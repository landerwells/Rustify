use crate::audio_thread::AudioCommand;
use crate::TemplateApp;

use eframe::egui; // Make sure to import necessary modules // Import your app struct if needed

pub fn show_central_panel(ctx: &egui::Context, app: &mut TemplateApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            // Determine the header text based on the current playlist
            let header_text = match &app.current_playlist {
                Some(playlist_name) => playlist_name,
                None => "All Songs",
            };
            ui.heading(header_text);

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for track in &app.track_list {
                    let button =
                        egui::Button::new(&track.title).fill(ui.style().visuals.window_fill());

                    let response = ui.add(button);

                    if response.clicked() {
                        app.audio_thread_sender
                            .send(AudioCommand::PlaySong(track.file_path.clone()))
                            .unwrap();
                    }

                    response.context_menu(|ui| {
                        // add to queue
                        if ui.button("Add to Queue").clicked() {
                            app.queue.add_track(track.clone());
                            // Find a way to close the context menu since I
                            // don't want it to stay open.
                            ui.close_menu();
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

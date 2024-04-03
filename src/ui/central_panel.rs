use crate::audio_thread::AudioCommand;
use crate::TemplateApp;
use eframe::egui;

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
                for track in &app.track_list.clone() {
                    let button =
                        egui::Button::new(&track.title).fill(ui.style().visuals.window_fill());

                    let response = ui.add(button);

                    if response.clicked() {
                        app.current_track = Some(track.file_path.clone());
                        app.audio_thread_sender
                            .send(AudioCommand::PlaySong(track.file_path.clone()))
                            .unwrap();
                    }

                    response.context_menu(|ui| {
                        if ui.button("Add to Queue").clicked() {
                            app.queue.add_track(track.clone());
                            ui.close_menu();
                        }
                        // Condition to separate whether the song should be added or removed from
                        // the playlist
                        if app.current_playlist.is_some() {
                            if ui.button("Remove from Playlist").clicked() {
                                if let Some(playlist_name) = &app.current_playlist {
                                    if let Some(playlist) = app
                                        .playlist_list
                                            .iter_mut()
                                            .find(|playlist| playlist.name == *playlist_name)
                                            {
                                                // need to find some way to update the track list
                                                playlist.remove_track(track.clone());
                                                while let Some(index) = app.track_list.iter().position(|x| x == track) {
                                                    app.track_list.remove(index);
                                                }
                                            }
                                }
                                ui.close_menu();
                            }
                        } else {
                            ui.menu_button("Add to Playlist", |ui| {
                                for playlist in &mut app.playlist_list {
                                    if ui.button(&playlist.name).clicked() {
                                        playlist.add_track(track.clone());
                                        ui.close_menu();
                                    }
                                }
                            });
                        }
                    });
                    ui.separator();
                }
            });
        });
    });
}

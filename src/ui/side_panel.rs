use crate::audio_track;
use crate::queue::Queue;
use crate::playlist::Playlist;
use crate::TemplateApp;

pub fn show_side_panel(ctx: &egui::Context, app: &mut TemplateApp) {
    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        ui.heading("Playlists");
        ui.separator();

        if ui.button("All Songs").clicked() {
            app.track_list = audio_track::get_tracks();
            app.current_playlist = None;
        }

        ui.separator();

        if ui.button("Add Playlist").clicked() {
            app.show_playlist_input = true;
        }

        ui.separator();

        if ui.button("Queue").clicked() {
            app.track_list = Queue::get_tracks(&app.queue);
            app.current_playlist = None;
        }

        if app.show_playlist_input {
            ui.label("Enter new playlist name:");
            if ui
                .text_edit_singleline(&mut app.new_playlist_name)
                .changed()
            {
                app.playlist_creation_error = None; // Clear the error when the user starts typing
            }

            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
            if enter_pressed || ui.button("Create").clicked() {
                if app
                    .playlist_list
                    .iter()
                    .any(|p| p.name == app.new_playlist_name)
                {
                    app.playlist_creation_error =
                        Some("Playlist with this name already exists.".to_string());
                } else if !app.new_playlist_name.is_empty() {
                    app.playlist_list
                        .push(Playlist::new(app.new_playlist_name.clone()));
                    app.new_playlist_name.clear();
                    app.show_playlist_input = false;
                }
            }

            if ui.button("Cancel").clicked() {
                app.new_playlist_name.clear();
                app.show_playlist_input = false;
                app.playlist_creation_error = None;
            }

            if let Some(error) = &app.playlist_creation_error {
                ui.colored_label(egui::Color32::RED, error);
            }
        }

        ui.separator();

        let mut playlists_to_delete: Vec<String> = Vec::new();

        for playlist in &app.playlist_list {
            let button = ui.button(&playlist.name);

            if button.clicked() {
                app.track_list = playlist.tracks.clone();
                app.current_playlist = Some(playlist.name.clone());
            }

            button.context_menu(|ui| {
                if ui.button("Delete Playlist").clicked() {
                    playlists_to_delete.push(playlist.name.clone());
                    app.track_list = audio_track::get_tracks();
                }
            });
        }
        app.playlist_list
            .retain(|p| !playlists_to_delete.contains(&p.name));
    });
}

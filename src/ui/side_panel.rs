use crate::audio_track;
use crate::playlist::Playlist;
use crate::TemplateApp;

// Getting some coupling here, would like to extract the more technical
// details into the playlist module but not for right now.
pub fn show_side_panel(ctx: &egui::Context, app: &mut TemplateApp) {
    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        ui.heading("Playlists");
        ui.separator();

        if ui.button("All Songs").clicked() {
            app.track_list = audio_track::get_tracks();
        }

        if ui.button("Add Playlist").clicked() {
            app.show_playlist_input = true;
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
                // Left-click logic
                // e.g., Display songs in this playlist in the central display
            }

            button.context_menu(|ui| {
                if ui.button("Delete Playlist").clicked() {
                    playlists_to_delete.push(playlist.name.clone());
                }
            });
        }
        app.playlist_list
            .retain(|p| !playlists_to_delete.contains(&p.name));
    });
}

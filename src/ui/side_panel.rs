use crate::playlist::Playlist;
use crate::TemplateApp;

// Getting some coupling here, would like to extract the more technical
// details into the playlist module but not for right now.
pub fn show_side_panel(ctx: &egui::Context, app: &mut TemplateApp) {
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
            app.playlist_list.push(playlist);
        }
        ui.separator();

        let mut playlists_to_delete: Vec<String> = Vec::new();

        for playlist in &app.playlist_list {
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
        app.playlist_list
            .retain(|p| !playlists_to_delete.contains(&p.name));
    });
}

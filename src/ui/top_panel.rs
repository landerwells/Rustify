use crate::TemplateApp;

pub fn show_top_panel(ctx: &egui::Context, _frame: &mut eframe::Frame, _app: &mut TemplateApp) {
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
}

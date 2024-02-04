use crate::audio_thread::AudioCommand;
use crate::audio_thread::AudioState;
use crate::playlist::Playlist;
use crate::TemplateApp;

pub fn show_bottom_panel(ctx: &egui::Context, app: &mut TemplateApp) {
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            // Volume slider
            // let mut volume = self.volume; // Assuming `self.volume` holds the current volume
            ui.add(egui::Slider::new(&mut app.volume, 0.0..=1.0).text("Volume"));
            app.audio_thread_sender
                .send(AudioCommand::SetVolume(app.volume))
                .unwrap();

            let button_label = if app.is_playing { "⏸" } else { "▶" };
            if ui.button(button_label).clicked() {
                let (state_sender, state_receiver) = std::sync::mpsc::channel();
                app.audio_thread_sender
                    .send(AudioCommand::GetState(state_sender))
                    .unwrap();

                // Receive the current state
                match state_receiver.recv() {
                    Ok(AudioState::Playing) => {
                        app.audio_thread_sender.send(AudioCommand::Pause).unwrap();
                    }
                    Ok(AudioState::Paused) => {
                        app.audio_thread_sender.send(AudioCommand::Play).unwrap();
                    }
                    _ => (),
                }
            }
            if ui.button("⏭").clicked() {
                app.audio_thread_sender.send(AudioCommand::Skip).unwrap();
            }

            if ui
                .add(egui::Slider::new(&mut app.track_progress, 0.0..=1.0).text("Track Progress"))
                .changed()
            {
                // This block will only execute if the slider's value has changed
                app.audio_thread_sender
                    .send(AudioCommand::SetProgress(app.track_progress))
                    .unwrap();
            }
        });
    });
}

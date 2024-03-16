use crate::audio_thread::AudioCommand;
use crate::audio_thread::AudioState;
use crate::TemplateApp;

pub fn show_bottom_panel(ctx: &egui::Context, app: &mut TemplateApp) {
    ctx.request_repaint();
    let (state_sender, state_receiver) = std::sync::mpsc::channel();
    app.audio_thread_sender
        .send(AudioCommand::GetState(state_sender))
        .unwrap();

    // Receive the current state
    match state_receiver.recv() {
        Ok(AudioState::Playing) => {
            app.audio_state = AudioState::Playing;
        }
        Ok(AudioState::Paused) => {
            app.audio_state = AudioState::Paused;
        }
        Ok(AudioState::Empty) => {
            if app.queue.tracks.len() > 0 {
                app.audio_state = AudioState::Playing;
                app.audio_thread_sender
                    .send(AudioCommand::PlaySong(
                        app.queue.tracks[0].file_path.clone(),
                    ))
                    .unwrap();
                app.queue.tracks.remove(0);
            } else {
                app.audio_state = AudioState::Empty;
            }
        }
        _ => (),
    }

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0; // Adjust spacing as needed

            // Volume slider
            ui.label("Volume:"); // Optionally, add a label for clarity
            ui.add(egui::Slider::new(&mut app.volume, 0.0..=1.0));
            app.audio_thread_sender
                .send(AudioCommand::SetVolume(app.volume))
                .unwrap();

            ui.separator();

            let button_label = if app.audio_state == AudioState::Playing {
                "⏸"
            } else {
                "▶"
            };

            if ui.button(button_label).clicked() {
                let (state_sender, state_receiver) = std::sync::mpsc::channel();
                app.audio_thread_sender
                    .send(AudioCommand::GetState(state_sender))
                    .unwrap();

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

            ui.separator();

            if ui.button("⏭").clicked() {
                app.audio_thread_sender.send(AudioCommand::Skip).unwrap();
            }

            ui.separator();

            let (progress_sender, progress_reciever) = std::sync::mpsc::channel();
            app.audio_thread_sender
                .send(AudioCommand::GetProgress(progress_sender))
                .unwrap();

            let progress = progress_reciever.recv().unwrap();
            // Convert progress from a Duration to a f32
            app.track_progress = progress.as_secs_f32();

            let (duration_sender, duration_reciever) = std::sync::mpsc::channel();
            app.audio_thread_sender
                .send(AudioCommand::GetTrackDuration(duration_sender))
                .unwrap();

            let duration = duration_reciever.recv().unwrap();
            app.track_duration = duration.as_secs_f32();

            if ui
                .add(
                    egui::Slider::new(&mut app.track_progress, 0.0..=app.track_duration)
                        .text("Track Progress"),
                )
                .changed()
            {
                // if no current track do nothing
                if app.current_track.is_none() {
                    return;
                }
                app.audio_thread_sender
                    .send(AudioCommand::SetProgress(
                        app.track_progress,
                        app.current_track.clone().unwrap(),
                    ))
                    .unwrap();
            }

        });
    });
}

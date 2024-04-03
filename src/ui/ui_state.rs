use crate::audio_thread::AudioCommand;
use crate::audio_thread::AudioState;
use crate::TemplateApp;

pub fn update_app_state(app: &mut TemplateApp) {
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
                app.current_track = None;
            }
        }
        _ => (),
    }
}

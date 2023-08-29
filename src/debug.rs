use std::env;

use ui::{AppWindow, ComponentHandle, GlobalState};

/// Set all interval durations to just a few seconds if DEBUG_DURATIONS
/// environment variable is set. Useful for debugging purposes so you
/// don't have to wait for too long
pub fn apply_debug_durations(app: &AppWindow) {
    if env::var("DEBUG_DURATIONS").is_ok() {
        let state = app.global::<GlobalState>();
        let mut settings = state.get_settings();
        settings.pomodoro_duration = 5000;
        settings.short_break_duration = 3000;
        settings.long_break_duration = 4000;
        state.set_settings(settings);
    }
}

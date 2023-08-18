pub mod settings;
pub mod styles;

use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use settings::Settings;
use ui::{AppWindow, ComponentHandle, GlobalCallbacks, GlobalState, Status};

fn main() {
    let app = AppWindow::new().unwrap();

    // Setup
    let state = app.global::<GlobalState>();
    if let Ok(settings) = Settings::load()  {
        state.set_settings(settings.into());
    }

    // Callbacks
    let callbacks = app.global::<GlobalCallbacks>();
    let app_clone = app.clone_strong();
    callbacks.on_start_timer(move || {
        let state = app_clone.global::<GlobalState>();
        state.set_status(Status::Work);
        state.set_timer_start_time(current_time());
        state.set_current_time(current_time());
    });
    let app_clone = app.clone_strong();
    callbacks.on_stop_timer(move || {
        let state = app_clone.global::<GlobalState>();
        state.set_status(Status::Stopped);
    });
    let app_clone = app.clone_strong();
    callbacks.on_settings_update(move || {
        let state = app_clone.global::<GlobalState>();
        let settings: Settings = state.get_settings().into();
        // Don't block the UI thread
        thread::spawn(move || {
            settings.save().ok();
        });
    });

    // Background jobs
    let app_clone = app.clone_strong();
    ui::spawn_local(async move {
        let state = app_clone.global::<GlobalState>();
        loop {
            state.set_current_time(current_time());

            if current_time() > state.get_timer_start_time() + state.invoke_interval_duration() {
                match state.get_status() {
                    Status::Work => {
                        let pomodoros_completed = state.get_pomodoros_completed() + 1;
                        state.set_pomodoros_completed(pomodoros_completed);
                        if (pomodoros_completed % state.get_settings().pomodoros_in_cycle) == 0 {
                            state.set_status(Status::LongBreak);
                            state.set_timer_start_time(current_time());
                        } else {
                            state.set_status(Status::Break);
                            state.set_timer_start_time(current_time());
                        }
                    },
                    Status::Break | Status::LongBreak => {
                        state.set_status(Status::Work);
                        state.set_timer_start_time(current_time());
                    },
                    Status::Stopped => (),
                }
            }

            async_std::task::sleep(Duration::from_millis(100)).await;
        }
    })
    .unwrap();

    app.run().ok();
}

fn current_time() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}

pub mod audio;
pub mod barrier;
mod debug;
mod instance;
mod notifications;
mod settings;
mod tray;
pub mod utils;

use std::time::Duration;
use std::{env, thread};

use async_std::task;
use barrier::Barrier;
use instance::{Instance, InstanceError};
use notify_rust::{ActionResponse, Timeout};
use ui::{AppWindow, ComponentHandle, GlobalCallbacks, GlobalState, Status};

fn main() {
    let app = AppWindow::new().unwrap();
    settings::apply_saved_settings(&app);
    debug::apply_debug_durations(&app);
    if env::args().any(|s| s == "--install") {
        utils::install_desktop_files().ok();
    }
    tray::setup_tray(&app);

    // Callbacks
    let callbacks = app.global::<GlobalCallbacks>();

    let barrier = Barrier::new();
    callbacks.on_start_timer(clone_army!([app, barrier] move || {
        let state = app.global::<GlobalState>();
        if state.get_paused() {
            barrier.unlock();
        } else {
            state.set_status(Status::Work);
            state.set_timer_start_time(utils::current_time());
            state.set_current_time(utils::current_time());
        }
    }));

    callbacks.on_stop_timer(clone_army!([app, barrier] move || {
        let state = app.global::<GlobalState>();
        state.set_status(Status::Stopped);
        barrier.unlock();
    }));

    callbacks.on_settings_update(clone_army!([app] move || {
        settings::save_settings(&app);
    }));

    audio::apply_callbacks(callbacks);

    match Instance::new() {
        Ok(instance) => {
            let weak = app.as_weak();
            ui::spawn_local(async move {
                loop {
                    instance.wait_for_another_instance_launch().await.unwrap();
                    weak.upgrade_in_event_loop(|app| {
                        app.window().show().unwrap();
                    })
                    .unwrap();
                }
            })
            .unwrap();
        }
        Err(InstanceError::IsLocked) => {
            // TODO: Add flag to disable this and launch anyway
            println!("Another instance is already running, exiting");
            return;
        }
        Err(InstanceError::Other(e)) => println!("Error: {}", e),
    };

    ui::spawn_local(clone_army!([app, barrier] async move {
        let state = app.global::<GlobalState>();
        loop {
            state.set_current_time(utils::current_time());

            if utils::current_time()
                > state.get_timer_start_time() + state.invoke_interval_duration()
            {
                match state.get_status() {
                    Status::Work => {
                        notifications::notification()
                            .body("Go and get some rest!")
                            .finalize().show_async().await.ok();
                        ui::spawn_local(async {
                            audio::play_sound(audio::RING_SOUND).await;
                        }).ok();
                        let pomodoros_completed = state.get_pomodoros_completed() + 1;
                        state.set_pomodoros_completed(pomodoros_completed);
                        if (pomodoros_completed % state.get_settings().pomodoros_in_cycle) == 0 {
                            state.set_status(Status::LongBreak);
                        } else {
                            state.set_status(Status::Break);
                        }
                        state.set_timer_start_time(utils::current_time());
                    }
                    Status::Break | Status::LongBreak => {
                        state.set_paused(true);
                        ui::spawn_local(async {
                            audio::play_sound(audio::RING_SOUND).await;
                        }).ok();
                        let handle = match notifications::notification()
                            .summary("Time's up")
                            .body("Ready to continue working?")
                            .action("continue", "Continue")
                            .timeout(Timeout::Never)
                            .finalize()
                            .show_async().await {
                                Ok(handle) => {
                                    let id = handle.id();
                                    thread::spawn(clone_army!([barrier] move ||
                                    notify_rust::handle_action(id, |action| {
                                        if let ActionResponse::Custom("continue") = action {
                                            barrier.unlock()
                                        }
                                    })));
                                    Some(handle)
                                }
                                Err(_) => None,
                        };

                        barrier.wait().await;
                        if let Some(handle) = handle {
                            handle.close()
                        }
                        state.set_paused(false);
                        if state.get_status() == Status::Stopped {
                            continue;
                        }
                        state.set_status(Status::Work);
                        state.set_timer_start_time(utils::current_time());
                    }
                    Status::Stopped => (),
                }
            }

            task::sleep(Duration::from_millis(100)).await;
        }
    }))
    .unwrap();

    app.window().show().unwrap();
    let state = app.global::<GlobalState>();
    while !state.get_quit_app() {
        ui::run_event_loop().unwrap();
    }
}

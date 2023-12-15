use ui::{AppWindow, ComponentHandle, GlobalState};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::thread;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pomodoro_duration: i64,
    short_break_duration: i64,
    long_break_duration: i64,
    pomodoros_in_cycle: i32,
    sound_volume: i32,
    show_completed_pomodoros: bool,
}

pub fn apply_saved_settings(app: &AppWindow) {
    let state = app.global::<GlobalState>();
    if let Ok(settings) = Settings::load() {
        state.set_settings(settings.into());
    }
}

pub fn save_settings(app: &AppWindow) {
    let state = app.global::<GlobalState>();
    let settings: Settings = state.get_settings().into();
    // Don't block UI thread
    thread::spawn(move || {
        settings.save().ok();
    });
}

impl Settings {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        match ProjectDirs::from("com.github", "Vinegret43", env!("CARGO_PKG_NAME")) {
            Some(dirs) => {
                let config_dir = dirs.config_dir();
                if !config_dir.exists() {
                    fs::create_dir_all(config_dir)?;
                }
                Ok(serde_yaml::from_reader(File::open(
                    config_dir.join("config.yaml"),
                )?)?)
            }
            None => Err("Could not compute project dirs".into()),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        match ProjectDirs::from("com.github", "Vinegret43", env!("CARGO_PKG_NAME")) {
            Some(dirs) => {
                let config_dir = dirs.config_dir();
                if !config_dir.exists() {
                    fs::create_dir_all(config_dir)?;
                }
                Ok(serde_yaml::to_writer(
                    File::create(config_dir.join("config.yaml"))?,
                    self,
                )?)
            }
            None => Err("Could not compute project dirs".into()),
        }
    }
}

impl From<ui::Settings> for Settings {
    fn from(settings: ui::Settings) -> Self {
        Self {
            pomodoro_duration: settings.pomodoro_duration,
            short_break_duration: settings.short_break_duration,
            long_break_duration: settings.long_break_duration,
            pomodoros_in_cycle: settings.pomodoros_in_cycle,
            sound_volume: settings.sound_volume,
            show_completed_pomodoros: settings.show_completed_pomodoros,
        }
    }
}

impl From<Settings> for ui::Settings {
    fn from(val: Settings) -> ui::Settings {
        ui::Settings {
            pomodoro_duration: val.pomodoro_duration,
            short_break_duration: val.short_break_duration,
            long_break_duration: val.long_break_duration,
            pomodoros_in_cycle: val.pomodoros_in_cycle,
            sound_volume: val.sound_volume,
            show_completed_pomodoros: val.show_completed_pomodoros,
        }
    }
}

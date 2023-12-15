use crate::clone_army;

use std::io::Cursor;

use rodio::{source::ChannelVolume, Decoder, OutputStream, Source};

use ui::{AppWindow, ComponentHandle, GlobalCallbacks, GlobalState};

pub const CLICK_DOWN_SOUND: &[u8] = include_bytes!("../assets/audio/click_down.mp3");
pub const CLICK_UP_SOUND: &[u8] = include_bytes!("../assets/audio/click_up.mp3");
pub const RING_SOUND: &[u8] = include_bytes!("../assets/audio/ring.mp3");

pub fn apply_callbacks(app: &AppWindow) {
    let callbacks = app.global::<GlobalCallbacks>();
    callbacks.on_play_click_down_sound(clone_army!([app] move || {
        ui::spawn_local(clone_army!([app] async move {
            let state = app.global::<GlobalState>();
            play_sound(CLICK_DOWN_SOUND, state.get_settings().sound_volume as f32 / 100.0).await;
        }))
        .ok();
    }));

    callbacks.on_play_click_up_sound(clone_army!([app] move || {
        ui::spawn_local(clone_army!([app] async move {
            let state = app.global::<GlobalState>();
            play_sound(CLICK_UP_SOUND, state.get_settings().sound_volume as f32 / 100.0).await;
        }))
        .ok();
    }));
}

pub async fn play_sound(sound_raw: &'static [u8], volume: f32) {
    let reader = Cursor::new(sound_raw);
    let source = Decoder::new(reader).unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle
        .play_raw(ChannelVolume::new(source, vec![volume]).convert_samples())
        .unwrap();
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
}

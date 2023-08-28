use std::io::Cursor;

use rodio::{Decoder, OutputStream, Source};

use ui::GlobalCallbacks;

pub const CLICK_DOWN_SOUND: &[u8] = include_bytes!("../assets/audio/click_down.mp3");
pub const CLICK_UP_SOUND: &[u8] = include_bytes!("../assets/audio/click_up.mp3");

pub fn apply_callbacks(callbacks: GlobalCallbacks) {
    callbacks.on_play_click_down_sound(|| {
        ui::spawn_local(async {
            play_sound(CLICK_DOWN_SOUND).await;
        })
        .ok();
    });

    callbacks.on_play_click_up_sound(|| {
        ui::spawn_local(async {
            play_sound(CLICK_UP_SOUND).await;
        })
        .ok();
    });
}

pub async fn play_sound(sound_raw: &'static [u8]) {
    let reader = Cursor::new(sound_raw);
    let source = Decoder::new(reader).unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
}

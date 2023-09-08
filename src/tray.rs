use crate::clone_army;
use crate::utils;

use std::io::Cursor;

use async_std::{channel, task};
use dark_light::Mode;
use image::{codecs::png::PngDecoder, DynamicImage};
use tray_item::{IconSource, TrayItem};
use ui::{platform::WindowEvent, AppWindow, ComponentHandle, GlobalCallbacks, GlobalState, Status};

const ICON_DARK: &[u8] = include_bytes!("../assets/dispute_tray_dark.png");
const ICON_LIGHT: &[u8] = include_bytes!("../assets/dispute_tray_light.png");

enum TrayAction {
    StartTimer,
    StopTimer,
    Quit,
}

pub fn setup_tray(win: &AppWindow) {
    let icon = match dark_light::detect() {
        Mode::Dark => ICON_DARK,
        Mode::Light | Mode::Default => ICON_LIGHT,
    };
    let image = DynamicImage::from_decoder(PngDecoder::new(Cursor::new(icon)).unwrap()).unwrap();
    // KSNI accepts image in ARGB format instead of RGBA. On Windows/OSX this
    // may be different since they don't use KSNI
    let data = utils::rgba_into_argb(image.into_rgba8().into_vec());
    let mut tray = TrayItem::new(
        "Dispute",
        IconSource::Data {
            data,
            height: 256,
            width: 256,
        },
    )
    .unwrap();
    let (tx, rx) = channel::unbounded::<TrayAction>();
    ui::spawn_local(clone_army!([win] async move {
        let state = win.global::<GlobalState>();
        let callbacks = win.global::<GlobalCallbacks>();
        loop {
            match rx.recv().await {
                Ok(TrayAction::StartTimer) => {
                    if state.get_status() == Status::Stopped || state.get_paused() {
                        callbacks.invoke_start_timer();
                    }
                }
                Ok(TrayAction::StopTimer) => {
                    if state.get_status() != Status::Stopped {
                        callbacks.invoke_stop_timer();
                    }
                }
                Ok(TrayAction::Quit) => {
                    state.set_quit_app(true);
                    win.window().dispatch_event(WindowEvent::CloseRequested);
                    break;
                },
                Err(_) => break,
            }
        }
    }))
    .ok();
    tray.add_menu_item(
        "Start timer",
        clone_army!([tx] move || {
            task::block_on(tx.send(TrayAction::StartTimer)).ok();
        }),
    )
    .ok();
    tray.add_menu_item(
        "Stop timer",
        clone_army!([tx] move || {
            task::block_on(tx.send(TrayAction::StopTimer)).ok();
        }),
    )
    .ok();
    tray.add_menu_item(
        "Quit",
        clone_army!([tx] move || {
            task::block_on(tx.send(TrayAction::Quit)).ok();
        }),
    )
    .ok();
}

use crate::utils;
use notify_rust::Notification;

pub fn notification() -> Notification {
    Notification::new()
        .appname(&utils::capitalize(env!("CARGO_PKG_NAME")))
        .finalize()
}

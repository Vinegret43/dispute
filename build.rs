#[cfg(target_family = "windows")]
use windres::Build;

#[cfg(target_family = "windows")]
fn main() {
    Build::new().compile("resources.rc").unwrap();
}

#[cfg(target_family = "unix")]
fn main() {}

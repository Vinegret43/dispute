pub use slint::*;

slint::include_modules!();

impl Clone for AppWindow {
    fn clone(&self) -> Self {
        self.clone_strong()
    }
}

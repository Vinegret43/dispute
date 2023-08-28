use async_std::sync::{Condvar, Mutex};
use std::sync::Arc;

#[derive(Clone)]
pub struct Barrier {
    condvar: Arc<Condvar>,
}

impl Barrier {
    pub fn new() -> Self {
        Self {
            condvar: Arc::new(Condvar::new()),
        }
    }

    pub fn unlock(&self) {
        self.condvar.notify_all();
    }

    pub async fn wait(&self) {
        self.condvar.wait(Mutex::new(()).lock().await).await;
    }
}

impl Default for Barrier {
    fn default() -> Self {
        Self::new()
    }
}

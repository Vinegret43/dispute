use crate::Barrier;

use std::fs::{self, File};
use std::path::PathBuf;

use directories::ProjectDirs;
use fslock::LockFile;
use notify::{Event, EventKind, RecursiveMode, Watcher};

/// To avoid launching many instances of the app. When another instance tries
/// to open, it will know whether an old one is already running and notify it
/// (Might be useful to, for example, re-open the window if it is closed)
pub struct Instance {
    _lock: LockFile,
    _watcher: Box<dyn Watcher + Send>,
    barrier: Barrier,
}

impl Instance {
    pub fn new() -> Result<Instance, InstanceError> {
        let path = Self::lock_file_path()?;
        if !path.exists() {
            let _fd = File::create(&path).ok();
        }
        let mut lock = LockFile::open(&path)?;
        if !lock.try_lock()? {
            return Err(InstanceError::IsLocked);
        };
        let barrier = Barrier::new();
        let barrier_clone = barrier.clone();
        let mut watcher =
            notify::recommended_watcher(move |event: notify::Result<Event>| match event {
                Ok(e) => match e.kind {
                    EventKind::Access(_) => barrier_clone.unlock(),
                    _ => (),
                },
                Err(_) => return,
            })?;
        watcher.watch(&path, RecursiveMode::NonRecursive)?;
        Ok(Self {
            _lock: lock,
            _watcher: Box::new(watcher),
            barrier,
        })
    }

    pub async fn wait_for_another_instance_launch(&self) -> std::io::Result<()> {
        self.barrier.wait().await;
        Ok(())
    }

    fn lock_file_path() -> Result<PathBuf, InstanceError> {
        let dir = match ProjectDirs::from("com.github", "Vinegret43", env!("CARGO_PKG_NAME")) {
            Some(dirs) => dirs.data_dir().to_owned(),
            None => return Err(InstanceError::Other("Cannot get lock file path".into())),
        };
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(dir.join("session.lock"))
    }
}

pub enum InstanceError {
    IsLocked,
    Other(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for InstanceError {
    fn from(e: std::io::Error) -> Self {
        Self::Other(Box::new(e))
    }
}

impl From<notify::Error> for InstanceError {
    fn from(e: notify::Error) -> Self {
        Self::Other(Box::new(e))
    }
}

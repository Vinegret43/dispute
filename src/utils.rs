use directories::BaseDirs;

use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use std::{fs, env};

pub fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => String::from(first.to_ascii_uppercase()) + chars.as_str(),
    }
}


const DESKTOP_FILE: &str = include_str!("../assets/dispute.desktop");
const ICON: &[u8] = include_bytes!("../assets/dispute.png");
pub fn install_desktop_files() -> Result<(), Box<dyn std::error::Error>> {
    let dirs = BaseDirs::new().ok_or(DirsError {})?;
    let home = dirs.home_dir();
    let apps_dir = try_create_dir(home.join(".local/share/applications/"))?;
    let icons_dir = try_create_dir(home.join(".local/share/icons/"))?;
    let desktop_file_path = apps_dir.join("dispute.desktop");

    if let Ok(()) = fs::write(&desktop_file_path, DESKTOP_FILE.replace("{}", &format!("{}", env::current_exe()?.display()))) {
        println!("Created the desktop file at {}", desktop_file_path.display());
    }

    let icon_file_path = icons_dir.join("dispute.png");
    if let Ok(()) = fs::write(&icon_file_path, ICON) {
        println!("Created the icon file at {}", icon_file_path.display());
    }

    Ok(())
}

#[derive(Debug)]
struct DirsError {}

use std::fmt;
impl fmt::Display for DirsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not calculate home directory")
    }
}

impl std::error::Error for DirsError {}

fn try_create_dir(path: impl Into<PathBuf>) -> std::io::Result<PathBuf> {
    let path = path.into();
    match path.canonicalize() {
        Ok(path) => Ok(path),
        Err(_) => {
            fs::create_dir_all(&path)?;
            let path = PathBuf::from(&path).canonicalize()?;
            println!("Created dir at {}", path.display());
            Ok(path)
        }
    }
}

// Macro from durka/closet repo. Removes boilerplate when cloning variables
// for a `move` closure
#[macro_export]
macro_rules! clone_army {
    (@as_expr $e:expr) => { $e };

    ([$($var:ident),*] $cl:expr) => {{
        $(let $var = $var.clone();)*
            $cl
    }};
}

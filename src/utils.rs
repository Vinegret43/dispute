use std::time::{SystemTime, UNIX_EPOCH};
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

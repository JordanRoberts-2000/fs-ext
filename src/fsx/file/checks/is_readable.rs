use std::{fs, path::Path};

pub fn is_readable(path: &Path) -> bool {
    fs::File::open(path).is_ok()
}
